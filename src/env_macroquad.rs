/** ===============================================================================
 * File: env_piston.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements graphical 2D environment using `macroquad` graphics lib
 * ===============================================================================*/
use crate::creature::*;
use crate::environment;
use crate::environment::*;
use std::io::Write;
use std::fs::File;
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
const DEFAULT_START_CREATURES : usize = 250;
const DEFAULT_START_FOOD : usize = 500;
const DEFAULT_START_WALLS : usize = 250;

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
const CONTROL_PANEL_HEIGHT : f32 = 125.0 + PANEL_Y_PADDING;
const CONTROL_PANEL_WIDTH : f32 = SCREEN_SIZE_X + PANEL_X_PADDING;

// Creature display params
const ORIENTATION_LINE_THICKNESS : f32 = 2.0;

// Window Parameters
const WINDOW_BAR_HEIGHT : f32 = 0.0;
const WINDOW_HEIGHT_PX : f32 = WINDOW_BAR_HEIGHT + SCREEN_SIZE_Y + CONTROL_PANEL_HEIGHT + PANEL_Y_PADDING;
const WINDOW_WIDTH_PX : f32 = SCREEN_SIZE_X + STATS_PANEL_WIDTH + PANEL_X_PADDING;

// Sim defaults
const MACROQUAD_FRAME_TIME_S : f64 = 0.1;    // Time between sim steps for macroquad in seconds


//===============================================================================
// DATA
//===============================================================================

/// Parameters for this simulation
struct SimParameters {
    

    // String versions of the parameters for storing the text box versions
    pub env_x_size : String,                    // X size of the sim in "spaces"
    pub env_y_size : String,                    // Y size of the sim in "spaces"
    pub num_start_creatures : String,           // Number of creatures to start the sim with
    pub num_start_food : String,                // Number of starting food spaces
    pub num_start_walls : String,               // Number of starting wall spaces
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
    FASTFORWARD,    // Fast-forwarding to a target step in the simulation
}


/// Environment
pub struct EnvMacroquad {
    params : SimParameters,     // Constant values that sim is initialized with
    pub env : EnvironmentV1,    // Contains the whole environment

    // Sim state
    pub state : SimState,       // Current state of the sim (running/stopped)
    last_sim_update : f64,      // Time of last simulation update used when running to determine whether we should update
    step_to_jump_to : usize,    // Which step in the simulation we should jump to (if state is FASTFORWARD)
    step_to_jump_to_str : String, // String version of `step_to_jump_to` variable that holds

    // Environment derived parameters
    grid_x_size : f32,              // X size of a single grid square in pixels
    grid_y_size : f32,              // Y size of a single grid square in pixels

    // Layout parameters
    stats_panel_x_pos : f32,
    stats_panel_y_pos : f32,
    param_panel_x_pos : f32,
    param_panel_y_pos : f32,
    control_panel_x_pos : f32,
    control_panel_y_pos : f32,

    // Window params
    screen_width_pixels : f32,      // X Size of the environment in pixels
    screen_height_pixels : f32,     // Y size of the environment in pixels

    // Assets
    background_texture : Texture2D, // Background image texture
}


//===============================================================================
// FUNCTIONS
//===============================================================================

impl EnvMacroquad {

    /// Get a new instance of the Macroquad environment
    pub fn new() -> EnvMacroquad {

        // First set the screen size to default. Include the size of the stats panel
        request_new_screen_size(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX);

        // Initialize environment parameters
        let mut temp_env_params = EnvironmentParams::new(); 
        temp_env_params.env_x_size = NUM_GRID_SQUARES_X;
        temp_env_params.env_y_size = NUM_GRID_SQUARES_Y;
        temp_env_params.num_start_creatures = DEFAULT_START_CREATURES;
        temp_env_params.num_start_food = DEFAULT_START_FOOD;
        temp_env_params.num_start_walls = DEFAULT_START_WALLS;
        // - the rest of the parameters are just the environment default...

        let mut temp_env = EnvMacroquad {
            params : SimParameters {
                env_x_size : String::new(),
                env_y_size : String::new(),
                num_start_creatures : String::new(),
                num_start_food : String::new(),
                num_start_walls : String::new(),
                energy_per_food_piece : String::new(),
                max_offspring_per_reproduce : String::new(),
                mutation_prob : String::new(),
                avg_new_food_per_day : String::new(),
            },

            // Generate the environment given the parameters
            env : EnvironmentV1::new_rand(&temp_env_params),

            // State
            state : SimState::RUNNING,
            last_sim_update : get_time(),
            step_to_jump_to : 0,
            step_to_jump_to_str : String::new(),

            // Environment display params
            grid_x_size : SCREEN_SIZE_X / (NUM_GRID_SQUARES_X as f32),
            grid_y_size : SCREEN_SIZE_Y / (NUM_GRID_SQUARES_Y as f32),

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

            // background_image
            background_texture : Texture2D::from_file_with_format(include_bytes!("../data/grass_texture.png"), Some(ImageFormat::Png)),
        };

        // Populate initial param strings with values from sim
        temp_env.repopulate_parameter_strings();


        return temp_env;
    }

    /// Generate a new environment given the parameter values in the text boxes
    pub fn generate_new_environment(&mut self) {

        // Populate the parameters with values from text boxes
        if self.update_params_from_text() {

            // Generate a new environment with new params
            self.env = environment::EnvironmentV1::new_rand(&self.env.params);
        }
    }

    /// Run and display the next step of the simulation
    pub fn run_next_step(&mut self) {
        self.env.advance_step();

        // Print out status of creatures per step
        if DEBUG_LEVEL > 0 {
            self.env.show_all_creature_info()
        }
    }

    /// Save the 
    fn save_environment(&self, filename : String) {
        let mut json_file = File::create(filename).unwrap();
        json_file.write(self.env.to_json().as_bytes()).expect("Error writing environment to file!");
    }

    /// Update the simulation env board
    fn update_sim_display(&self) {

        // Draw background
        draw_texture(&self.background_texture, 0.0, 0.0, WHITE);

        // For each simulation space on the board, update with proper piece
        for x in 0..self.env.params.env_x_size {
            for y in 0..self.env.params.env_y_size {
                match self.env.positions[x][y] {
                    SpaceStates::CreatureSpace(id) => {
                        let c_id = self.env.get_creature_idx_from_id(id).unwrap(); 
                        let creature : &CreatureV1 = &self.env.creatures[c_id];
                        self.draw_creature_square(x, y, creature.orientation, creature.color);
                    }
                    SpaceStates::FoodSpace => self.draw_food_space(x, y),
                    SpaceStates::WallSpace => self.draw_wall_space(x, y),
                    SpaceStates::FightSpace => self.draw_fight_space(x, y), 
                    SpaceStates::BlankSpace => (),
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
            let temp_skin = Skin {
                label_style : ui.style_builder().font_size(16).build(),
                button_style : ui.style_builder()
                    .color(Color {r: 0.5, g: 0.5, b: 0.5, a: 1.0})
                    .color_hovered(Color {r: 0.7, g: 0.7, b: 0.7, a: 1.0})
                    .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
                    .font_size(16)
                    .build(),
                ..ui.default_skin()
            };
            ui.push_skin(&temp_skin);

            ui.label(None, "CONTROL PANEL"); 
            ui.label(None, "");

            if ui.button(None, "START/STOP") {
                self.state = match self.state {
                    SimState::RUNNING => SimState::STOPPED,
                    SimState::STOPPED => SimState::RUNNING,
                    SimState::FASTFORWARD => SimState::STOPPED,
                }
            }

            // Jump to a particular step button
            ui.same_line(0.0);
            if ui.button(None, "JUMP TO STEP") {
                // If target step is reasonable, then enter fast forward mode
                if self.step_to_jump_to > self.env.time_step && self.step_to_jump_to > 0 && self.step_to_jump_to < 1000000 {
                    self.state = SimState::FASTFORWARD;
                }
            }

            // Text box that gets step to jump to
            ui.input_text(hash!(), "Step to Jump to", &mut self.step_to_jump_to_str);
            let res = self.step_to_jump_to_str.parse::<usize>();
            match res {
                Err(_e) => self.step_to_jump_to = 0,
                Ok(step_val) => self.step_to_jump_to = step_val,
            }

            // Button to save the current environment as a json file
            if ui.button(None, "SAVE ENVIRONMENT") {
                self.save_environment("data/saved_env.json".to_string());
            }
            // Button to save the current environment as a json file
            ui.same_line(0.0);
            if ui.button(None, "LOAD ENVIRONMENT") {
                // TODO: implement
            }

            ui.pop_skin();
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
                ui.input_text(hash!(), "Num Start Walls", &mut self.params.num_start_walls);
                ui.input_text(hash!(), "Energy per Food", &mut self.params.energy_per_food_piece);
                ui.input_text(hash!(), "Max offspring per Reproduce", &mut self.params.max_offspring_per_reproduce);
                ui.input_text(hash!(), "Mutation Probability", &mut self.params.mutation_prob);
                ui.input_text(hash!(), "Avg New Food per Step", &mut self.params.avg_new_food_per_day);

                // Add button to regenerate new environment
                if ui.button(None, "Generate New Random Environment") {
                    self.generate_new_environment();
                }
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
    fn draw_creature_square(&self, x_pos : usize, y_pos : usize, orientation : CreatureOrientation, color : CreatureColor) {

        let xpos_pix = (x_pos as f32) * self.grid_x_size;
        let ypos_pix = (y_pos as f32) * self.grid_y_size;

        // Draw the rectangle "body" of the creature
        draw_rectangle(xpos_pix, ypos_pix, self.grid_x_size, self.grid_y_size, Color::from_rgba(color.red, color.green, color.blue, 255));

        // Draw a short line to indicate which direction the creature is facing
        let x_gridsize_div_2 = self.grid_x_size / 2.0;
        let y_gridsize_div_2 = self.grid_y_size / 2.0;
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
        let food_color = Color {r: (FOOD_SPACE_COLOR[0] as f32) / 255.0, g: (FOOD_SPACE_COLOR[1] as f32) / 255.0, b : (FOOD_SPACE_COLOR[2] as f32) / 255.0 , a: 1.0};
        draw_rectangle((x_pos as f32) * self.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, food_color);
    }

    /// Draw a wall space on the screen
    fn draw_wall_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, BLACK);
    }

    /// Draw a single food space on the screen
    fn draw_fight_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, Color {r: 1.0, g: 0.0, b: 0.0, a: 0.25});
    }

    /// Update the temporary parameter strings that param panel is populated from with the
    /// actual values from the environment
    fn repopulate_parameter_strings(&mut self) {
        self.params.env_x_size = format!("{}", self.env.params.env_x_size); 
        self.params.env_y_size = format!("{}", self.env.params.env_y_size); 
        self.params.num_start_creatures = format!("{}", self.env.params.num_start_creatures); 
        self.params.num_start_food = format!("{}", self.env.params.num_start_food); 
        self.params.num_start_walls = format!("{}", self.env.params.num_start_walls); 
        self.params.energy_per_food_piece = format!("{}", self.env.params.energy_per_food_piece); 
        self.params.max_offspring_per_reproduce = format!("{}", self.env.params.max_offspring_per_reproduce); 
        self.params.mutation_prob = format!("{}", self.env.params.mutation_prob); 
        self.params.avg_new_food_per_day = format!("{}", self.env.params.avg_new_food_per_day); 
    }

    /// Update the environment parameters from the values that are in the text boxes
    /// This should be called right before a new simulation is set up/generated
    /// Function returns True if all parameters are updated and false if any parameter is invalid
    /// or cannot be parsed
    fn update_params_from_text(&mut self) -> bool {
        // Create temporary params struct to validate everything before we apply it
        let mut temp_params = EnvironmentParams::new();

        // Parse the text to make sure that at least works
        temp_params.env_x_size = self.params.env_x_size.parse::<usize>().expect("Error parsing env_x_size");
        temp_params.env_y_size = self.params.env_x_size.parse::<usize>().expect("Error parsing env_y_size");
        temp_params.num_start_creatures = self.params.num_start_creatures.parse::<usize>().expect("Error parsing num_start_creatures");
        temp_params.num_start_food = self.params.num_start_food.parse::<usize>().expect("Error parsing num_start_food");
        temp_params.num_start_walls = self.params.num_start_walls.parse::<usize>().expect("Error parsing num_start_walls");
        temp_params.energy_per_food_piece = self.params.energy_per_food_piece.parse::<usize>().expect("Error parsing energy_per_food_piece");
        temp_params.max_offspring_per_reproduce = self.params.max_offspring_per_reproduce.parse::<usize>().expect("Error parsing max_offspring_per_reproduce");
        temp_params.mutation_prob = self.params.mutation_prob.parse::<f32>().expect("Error parsing mutation_prob");
        temp_params.avg_new_food_per_day = self.params.avg_new_food_per_day.parse::<f32>().expect("Error parsing avg_new_food_per_day");

        let num_spaces = temp_params.env_x_size * temp_params.env_y_size;

        // Validate a few things
        if temp_params.env_x_size > 10000 || temp_params.env_y_size > 10000 {
            println!("Error, invalid environment size specified");
            return false;
        }
        if temp_params.mutation_prob > 1.0 || temp_params.mutation_prob < 0.0 {
            println!("Error: mutation_prob is invalid. Must be between 0 and 1");
            return false;
        }
        if temp_params.num_start_food > num_spaces || temp_params.num_start_creatures > num_spaces || temp_params.num_start_walls > num_spaces {
            println!("Error: number of start food/creatures/walls is too large for a {} x {} grid", temp_params.env_x_size, temp_params.env_y_size);
            return false;
        }

        // Update some internal macroquad variables whos values are derived from env variables
        self.grid_x_size = SCREEN_SIZE_X / (temp_params.env_x_size as f32);
        self.grid_y_size = SCREEN_SIZE_Y / (temp_params.env_y_size as f32);

        // All good, copy the temp params into the real one
        self.env.params = temp_params;

        return true;
    }

    /// Update the display for fast forward mode
    /// basically just update the popup and run the simulation
    fn update_ff_mode(&mut self) {
        const NUM_STEPS_PER_CALL : usize = 100;

        // Run several steps
        let steps_to_go = self.step_to_jump_to - self.env.time_step;
        let res : Result<(), EnvErrors>;
        if (steps_to_go >= NUM_STEPS_PER_CALL) {
            res = self.env.run_n_steps(NUM_STEPS_PER_CALL);
        } else {
            res = self.env.run_n_steps(steps_to_go);
        }

        // If we couldn't run the sim, just stop
        match res {
            Err(e) => self.state = SimState::STOPPED,
            Ok(_) => (),
        }

        // Check for done condition
        if self.env.time_step >= self.step_to_jump_to {
            self.state = SimState::STOPPED;
        }

        // Update the popup
        root_ui().popup(hash!(), vec2(SCREEN_SIZE_X, SCREEN_SIZE_Y), |ui| {
            // Style the popup
            let popup_skin = Skin {
                label_style : ui.style_builder()
                    .text_color(WHITE)
                    .font_size(30)
                    .build(),
                ..ui.default_skin()
            };

            ui.push_skin(&popup_skin);
            ui.label(vec2(SCREEN_SIZE_X / 3.0, SCREEN_SIZE_Y / 3.0), &format!("Running step {} / {}", self.env.time_step, self.step_to_jump_to));
            ui.pop_skin();
        });
    }

    /// Call this every loop through the main async function
    pub fn main_loop(&mut self) {
        let mut cur_time = get_time();

        // If we're in fast forward mode, then simply run through this as fast as possible without updating display
        if self.state == SimState::FASTFORWARD {
            self.update_ff_mode();
        }

        // Update display every time through
        self.update_display();

        // Update current time
        cur_time = get_time();

        // Decide whether we should run the next sim step
        if (self.state == SimState::RUNNING) && (cur_time - self.last_sim_update > MACROQUAD_FRAME_TIME_S) {
            self.run_next_step();
            self.last_sim_update = get_time();
        }

    }

}


