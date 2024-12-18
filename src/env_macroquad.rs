/** ===============================================================================
 * File: env_piston.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements graphical 2D environment using `macroquad` graphics lib
 * ===============================================================================*/
use crate::creature::*;
use crate::environment::*;
use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,Skin,
    widgets::{self, Group},
    Drag, Ui,
};

//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEBUG_LEVEL : usize = 0;

// Size of the board
const SCREEN_SIZE_X : f32 = 800.0;
const SCREEN_SIZE_Y : f32 = 800.0;
const NUM_GRID_SQUARES_X : usize = 100;
const NUM_GRID_SQUARES_Y : usize = 100;


// Default environment parameters
const DEFAULT_START_CREATURES : usize = 500;
const DEFAULT_START_FOOD : usize = 500;
const DEFAULT_START_WALLS : usize = 100;

// Stat Panel params
const STATS_PANEL_WIDTH : f32 = 400.0;
const PANEL_X_PADDING : f32 = 10.0;
const PANEL_Y_PADDING : f32 = 10.0;
const STATS_PANEL_HEIGHT : f32 = WINDOW_HEIGHT_PX / 2.0;
const STATS_BACKGROUND_COLOR : Color = Color {r: 0.8, g: 0.8, b:0.8, a: 1.0};
const MAX_CREATURES_STATS_TO_DISPLAY : usize = 25;

// Param panel params
const PARAM_PANEL_WIDTH : f32 = 400.0;
const PARAM_PANEL_HEIGHT : f32 = WINDOW_HEIGHT_PX / 2.0;

// Control panel params (sits below the main board)
const CONTROL_PANEL_HEIGHT : f32 = 150.0 + PANEL_Y_PADDING;
const CONTROL_PANEL_WIDTH : f32 = SCREEN_SIZE_X + PANEL_X_PADDING;

// Creature display params
const ORIENTATION_LINE_THICKNESS : f32 = 2.0;

// Window Parameters
const WINDOW_BAR_HEIGHT : f32 = 0.0;
const WINDOW_HEIGHT_PX : f32 = WINDOW_BAR_HEIGHT + SCREEN_SIZE_Y + CONTROL_PANEL_HEIGHT + PANEL_Y_PADDING;
const WINDOW_WIDTH_PX : f32 = SCREEN_SIZE_X + STATS_PANEL_WIDTH + PANEL_X_PADDING;


//===============================================================================
// DATA
//===============================================================================

/// Parameters for this simulation
struct SimParameters {
    grid_x_size : f32,              // X size of a single grid square in pixels
    grid_y_size : f32,              // Y size of a single grid square in pixels

    // String versions of the parameters for storing the text box versions
    pub env_x_size : String,                    // X size of the sim in "spaces"
    pub env_y_size : String,                    // Y size of the sim in "spaces"
    pub num_start_creatures : String,           // Number of creatures to start the sim with
    pub num_start_food : String,                // Number of starting food spaces
    pub energy_per_food_piece : String,         // Number of energy units that will be given per food consumed 
    pub max_offspring_per_reproduce : String,   // Maximum number of offspring that will be produced by one reproduction event
    pub mutation_prob : String,                 // Probability that a single value in the creatures DNA will randomly mutate upon reproduction
    pub avg_new_food_per_day : String,          // Average number of new food pieces added to the environment per day

}

/// Enum defining state of the simulation (stopped/running)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SimState {
    RUNNING,
    STOPPED,
}


/// Environment
pub struct EnvMacroquad {
    params : SimParameters,     // Constant values that sim is initialized with
    pub env : EnvironmentV1,    // Contains the whole environment

    // Sim state
    pub state : SimState,       // Current state of the sim (running/stopped)

    stats_panel_x_pos : f32,
    stats_panel_y_pos : f32,
    param_panel_x_pos : f32,
    param_panel_y_pos : f32,
    control_panel_x_pos : f32,
    control_panel_y_pos : f32,

    screen_width_pixels : f32,      // X Size of the environment in pixels
    screen_height_pixels : f32,     // Y size of the environment in pixels

}


//===============================================================================
// FUNCTIONS
//===============================================================================

impl EnvMacroquad {

    /// Get a new instance of the Macroquad environment
    pub fn new() -> EnvMacroquad {

        // First set the screen size to default. Include the size of the stats panel
        request_new_screen_size(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX);

        let mut temp_env = EnvMacroquad {
            params : SimParameters {
                grid_x_size : SCREEN_SIZE_X / (NUM_GRID_SQUARES_X as f32),
                grid_y_size : SCREEN_SIZE_Y / (NUM_GRID_SQUARES_Y as f32),
                env_x_size : String::new(),
                env_y_size : String::new(),
                num_start_creatures : String::new(),
                num_start_food : String::new(),
                energy_per_food_piece : String::new(),
                max_offspring_per_reproduce : String::new(),
                mutation_prob : String::new(),
                avg_new_food_per_day : String::new(),
            },
            env : EnvironmentV1::new_rand(
                NUM_GRID_SQUARES_X, // env_x_size
                NUM_GRID_SQUARES_Y, // env_y_size
                DEFAULT_START_CREATURES, // num_start_creatures
                DEFAULT_START_FOOD, // num_start_food
                DEFAULT_START_WALLS, // num_walls
            ),

            // State
            state : SimState::RUNNING,

            // Set position of stats panel
            stats_panel_x_pos : SCREEN_SIZE_X + PANEL_X_PADDING,
            stats_panel_y_pos : 0.0,
            param_panel_x_pos : SCREEN_SIZE_X + PANEL_X_PADDING,
            param_panel_y_pos : STATS_PANEL_HEIGHT,
            control_panel_x_pos : 0.0,
            control_panel_y_pos : SCREEN_SIZE_X + PANEL_Y_PADDING,

            // Set total size of the window for internal tracking
            screen_width_pixels : WINDOW_WIDTH_PX,
            screen_height_pixels : WINDOW_HEIGHT_PX,
        };

        // Populate initial param strings with values from sim
        temp_env.repopulate_parameter_strings();

        return temp_env;
    }

    /// Run and display the next step of the simulation
    pub fn run_next_step(&mut self) {
        self.env.advance_step();

        // Print out status of creatures per step
        if DEBUG_LEVEL > 0 {
            self.env.show_all_creature_info()
        }
    }

    /// Update the simulation env board
    fn update_sim_display(&self) {

        // For each simulation space on the board, update with proper piece
        for x in 0..self.env.env_x_size {
            for y in 0..self.env.env_y_size {
                match self.env.positions[x][y] {
                    SpaceStates::CreatureSpace(id) => {
                        let c_id = self.env.get_creature_idx_from_id(id).unwrap(); 
                        let creature : &CreatureV1 = &self.env.creatures[c_id];
                        self.draw_creature_square(x, y, creature.orientation);
                    }
                    SpaceStates::FoodSpace => self.draw_food_space(x, y),
                    SpaceStates::WallSpace => self.draw_wall_space(x, y),
                    _ => (),
                }
            }
        }
    }

    /// Update the statistics panel
    fn update_stats_panel(&mut self) {
        const HEADER_FONT_SIZE_PX : f32 = 14.0;
        const MAIN_FONT_SIZE_PX : f32 = 10.0;
        let mut cur_y_pos_px = self.stats_panel_y_pos + HEADER_FONT_SIZE_PX;

        // Define style of the stats panel first
        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(0, 0,0, 255))
            .font_size(14)
            .build();
        let stats_skin = Skin {
            label_style : label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&stats_skin);

        // Define the content of the stats panel
        root_ui().window(hash!(), vec2(self.stats_panel_x_pos, self.stats_panel_y_pos), vec2(STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT), |ui| {
            ui.label(None, "SIMULATION STATISTICS"); 
            ui.label(None, ""); 
            let mut stat_txt = format!("{:22} {:<12?}", "STATE:", self.state);
            ui.label(None, &stat_txt);
            stat_txt = format!("{:22} {:<12}", "TIME STEP:", self.env.time_step);
            ui.label(None, &stat_txt); 
            stat_txt = format!("{:22} {:<12}", "TOTAL CREATURES:", self.env.num_total_creatures);
            ui.label(None, &stat_txt); 
            stat_txt = format!("{:22} {:<12}", "CURRENT CREATURES:", self.env.num_creatures);
            ui.label(None, &stat_txt); 
            stat_txt = format!("{:22} {:<12}", "NUM FOOD:", self.env.num_food);
            ui.label(None, &stat_txt); 
            stat_txt = format!("{:22} {:<12}", "NUM WALLS:", self.env.num_walls);
            ui.label(None, &stat_txt); 

            ui.label(None, "\n"); 
            ui.separator();

            let header_str = format!("{:12} {:12} {:12} {:15} ", "Creature Id", "Age", "Energy", "Last Action");
            ui.label(None, &header_str);

            for creature_idx in 0..self.env.creatures.len() {
                let creature = &self.env.creatures[creature_idx];
                let creature_str = format!("{:<12} {:<12} {:<12} {:<15?} ", creature.id, creature.age, creature.energy, creature.last_action);
                ui.label(None, &creature_str);

                if creature_idx > MAX_CREATURES_STATS_TO_DISPLAY {
                    break;
                }
            }
        });

        // Undo the UI skin, so it can be set by another panel
        root_ui().pop_skin();

    }


    /// Create/update the control panel in the UI
    fn update_control_panel(&mut self) {

        // Define the content of the control panel
        root_ui().window(hash!(), vec2(self.control_panel_x_pos, self.control_panel_y_pos), vec2(CONTROL_PANEL_WIDTH, CONTROL_PANEL_HEIGHT), |ui| {
            ui.label(None, "CONTROL PANEL"); 
            ui.label(None, "");

            if ui.button(None, "START/STOP") {
                self.state = match self.state {
                    SimState::RUNNING => SimState::STOPPED,
                    SimState::STOPPED => SimState::RUNNING,
                }
            }
        });
    }

    /// Update the simulation parameters panel
    fn update_sim_param_panel(&mut self) {

        root_ui().window(hash!(), vec2(self.param_panel_x_pos, self.param_panel_y_pos), vec2(PARAM_PANEL_WIDTH, PARAM_PANEL_HEIGHT), |ui| {
                ui.label(None, "SIMULATION PARAMETERS");
                ui.input_text(hash!(), "Env X Size", &mut self.params.env_x_size);
                ui.input_text(hash!(), "Env Y Size", &mut self.params.env_y_size);
                ui.input_text(hash!(), "Num Start Creatures", &mut self.params.num_start_creatures);
                ui.input_text(hash!(), "Num Start Food", &mut self.params.num_start_food);
                ui.input_text(hash!(), "Energy per Food", &mut self.params.energy_per_food_piece);
                ui.input_text(hash!(), "Max offspring per Reproduce", &mut self.params.max_offspring_per_reproduce);
                ui.input_text(hash!(), "Mutation Probability", &mut self.params.mutation_prob);
                ui.input_text(hash!(), "Avg New Food per Step", &mut self.params.avg_new_food_per_day);
            });
    }


    /// Update the display
    pub fn update_display(&mut self) {
        clear_background(GRAY);

        // Update the main board
        self.update_sim_display();

        // Update statistics on the side
        self.update_stats_panel(); 

        // Update the simulation start parameters panel
        self.update_sim_param_panel();

        // Update the control panel
        self.update_control_panel();
    }

    /// Draw a single creature square to the specified location on the screen
    fn draw_creature_square(&self, x_pos : usize, y_pos : usize, orientation : CreatureOrientation) {

        let xpos_pix = (x_pos as f32) * self.params.grid_x_size;
        let ypos_pix = (y_pos as f32) * self.params.grid_y_size;

        // Draw the rectangle "body" of the creature
        draw_rectangle(xpos_pix, ypos_pix, self.params.grid_x_size, self.params.grid_y_size, BLUE);

        // Draw a short line to indicate which direction the creature is facing
        let x_gridsize_div_2 = self.params.grid_x_size / 2.0;
        let y_gridsize_div_2 = self.params.grid_y_size / 2.0;
        let center_x = xpos_pix + x_gridsize_div_2;
        let center_y = ypos_pix + y_gridsize_div_2; 

        match orientation {
            CreatureOrientation::Up => draw_line(center_x, center_y, center_x, center_y - y_gridsize_div_2, ORIENTATION_LINE_THICKNESS, Color {r:0.0, g:0.0, b:0.0, a:1.0}),
            CreatureOrientation::Down => draw_line(center_x, center_y, center_x, center_y + y_gridsize_div_2, ORIENTATION_LINE_THICKNESS, Color {r:0.0, g:0.0, b:0.0, a:1.0}),
            CreatureOrientation::Left => draw_line(center_x, center_y, center_x - x_gridsize_div_2, center_y, ORIENTATION_LINE_THICKNESS, Color {r:0.0, g:0.0, b:0.0, a:1.0}),
            CreatureOrientation::Right => draw_line(center_x, center_y, center_x + x_gridsize_div_2, center_y, ORIENTATION_LINE_THICKNESS, Color {r:0.0, g:0.0, b:0.0, a:1.0}),
        }
    }

    /// Draw a single food space on the screen
    fn draw_food_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.params.grid_y_size, self.params.grid_x_size, self.params.grid_y_size, GREEN);
    }

    /// Draw a wall space on the screen
    fn draw_wall_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.params.grid_y_size, self.params.grid_x_size, self.params.grid_y_size, BLACK);
    }

    /// Update the temporary parameter strings that param panel is populated from with the
    /// actual values from the environment
    fn repopulate_parameter_strings(&mut self) {
        self.params.env_x_size = format!("{}", self.env.env_x_size); 
        self.params.env_y_size = format!("{}", self.env.env_y_size); 
        self.params.num_start_creatures = format!("{}", self.env.num_start_creatures); 
        self.params.num_start_food = format!("{}", self.env.num_start_food); 
        self.params.energy_per_food_piece = format!("{}", self.env.energy_per_food_piece); 
        self.params.max_offspring_per_reproduce = format!("{}", self.env.max_offspring_per_reproduce); 
        self.params.mutation_prob = format!("{}", self.env.mutation_prob); 
        self.params.avg_new_food_per_day = format!("{}", self.env.avg_new_food_per_day); 
    }

}


