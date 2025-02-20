/** ===============================================================================
 * File: env_piston.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements graphical 2D environment using `macroquad` graphics lib
 * ===============================================================================*/
use core_lib::creature::*;
use core_lib::environment::*;
use ::rand::Rng;
use std::io::Write;
use std::fs::File;

use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,Skin,
    widgets::{self},
};

//===============================================================================
// CONSTANTS
//===============================================================================

// Size of the board
const SCREEN_SIZE_X : f32 = 950.0;
const SCREEN_SIZE_Y : f32 = 700.0;

// Stat Panel params
const STATS_PANEL_WIDTH : f32 = 400.0;
const PANEL_X_PADDING : f32 = 10.0;
const PANEL_Y_PADDING : f32 = 10.0;
const STATS_PANEL_HEIGHT : f32 = WINDOW_HEIGHT_PX / 2.5;

// Param panel params
const PARAM_PANEL_WIDTH : f32 = 400.0;
const PARAM_PANEL_HEIGHT : f32 = WINDOW_HEIGHT_PX / 2.5;

// Control panel 1 (that sits on the right ide of the display)
const CONTROL1_PANEL_WIDTH : f32 = 400.0;
const CONTROL1_PANEL_HEIGHT : f32 = WINDOW_HEIGHT_PX / 5.0;

// Control panel params (sits below the main board)
const CONTROL2_PANEL_HEIGHT : f32 = 125.0 + PANEL_Y_PADDING;
const CONTROL2_PANEL_WIDTH : f32 = SCREEN_SIZE_X + PANEL_X_PADDING;

// Creature display params
const ORIENTATION_LINE_THICKNESS : f32 = 2.0;

// Window Parameters
const WINDOW_BAR_HEIGHT : f32 = 20.0;
const WINDOW_HEIGHT_PX : f32 = WINDOW_BAR_HEIGHT + SCREEN_SIZE_Y + CONTROL2_PANEL_HEIGHT + PANEL_Y_PADDING;
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

    pub save_load_filename : String,            // Name of file to save/load from
}

/// Enum defining state of the simulation (stopped/running)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SimState {
    RUNNING,
    STOPPED,
    FASTFORWARD,    // Fast-forwarding to a target step in the simulation
}

/// Can specify which parts of an environment to load from file
#[derive(Copy, Clone)]
pub struct LoadOptions {
    load_params : bool,
    load_creatures : bool,
    load_walls : bool,
    load_food : bool,
}

/// Environment
pub struct EnvMacroquad {
    params : SimParameters,     // Constant values that sim is initialized with
    pub env : Environment,    // Contains the whole environment

    // Sim state
    pub state : SimState,       // Current state of the sim (running/stopped)
    last_sim_update : f64,      // Time of last simulation update used when running to determine whether we should update
    step_to_jump_to : usize,    // Which step in the simulation we should jump to (if state is FASTFORWARD)
    step_to_jump_to_str : String, // String version of `step_to_jump_to` variable that holds

    // Data used by UI
    load_opts : LoadOptions,    // Options used when loading environment from a file

    // Data used to draw new spaces
    current_draw_space_type : Option<SpaceStates>,  // Current type of space that should be drawn if the user clicks on a space square

    // Environment derived parameters
    grid_x_size : f32,              // X size of a single grid square in pixels
    grid_y_size : f32,              // Y size of a single grid square in pixels

    // Layout parameters
    stats_panel_x_pos : f32,
    stats_panel_y_pos : f32,
    param_panel_x_pos : f32,
    param_panel_y_pos : f32,
    control1_panel_x_pos : f32,
    control1_panel_y_pos : f32,
    control_panel_x_pos : f32,
    control_panel_y_pos : f32,

    // Style skin
    default_skin : Skin,            // default sytle for the UI

    // Assets
    background_texture : Texture2D, // Background image texture
    background_options : DrawTextureParams,
}


//===============================================================================
// FUNCTIONS
//===============================================================================

impl EnvMacroquad {

    /// Get a new instance of the Macroquad environment
    pub fn new() -> EnvMacroquad {

        // First set the screen size to default. Include the size of the stats panel
        request_new_screen_size(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX);

        // Initialize environment parameters (just use default)
        let temp_env_params = EnvironmentParams::new(); 

        // Start with a randomly selected preset environment
        let mut rng = ::rand::thread_rng();
        let starting_env_num = rng.gen_range(0..NUM_DEFAULT_ENVS*2); // the `*2` just ensures that we have an equal chance of generating a random env and default

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
                save_load_filename : String::new(),
            },

            // Generate the environment given the parameters
            env : Environment::new_rand_from_default(&temp_env_params, Some(starting_env_num)),

            // State
            state : SimState::RUNNING,
            last_sim_update : get_time(),
            step_to_jump_to : 0,
            step_to_jump_to_str : String::new(),

            // UI data
            load_opts : LoadOptions {
                load_params : false,
                load_creatures : false,
                load_walls : false,
                load_food : false,
            },

            // Space drawing data
            current_draw_space_type : None,

            // Environment display params
            grid_x_size : SCREEN_SIZE_X / (temp_env_params.env_x_size as f32),
            grid_y_size : SCREEN_SIZE_Y / (temp_env_params.env_y_size as f32),

            // Set position of all info panels 
            stats_panel_x_pos : SCREEN_SIZE_X + PANEL_X_PADDING,
            stats_panel_y_pos : 0.0,
            param_panel_x_pos : SCREEN_SIZE_X + PANEL_X_PADDING,
            param_panel_y_pos : STATS_PANEL_HEIGHT + CONTROL1_PANEL_HEIGHT,
            control1_panel_x_pos : SCREEN_SIZE_X + PANEL_X_PADDING,
            control1_panel_y_pos : STATS_PANEL_HEIGHT,

            control_panel_x_pos : 0.0,
            control_panel_y_pos : SCREEN_SIZE_Y + PANEL_Y_PADDING,

            // Set default skin to default from macroquad (will be overwritten later)
            default_skin : Skin {..root_ui().default_skin()},

            // background_image
            background_texture : Texture2D::from_file_with_format(include_bytes!("../data/grass_texture.png"), Some(ImageFormat::Png)),
            background_options : DrawTextureParams {
                dest_size: Some(vec2(SCREEN_SIZE_X + PANEL_X_PADDING, SCREEN_SIZE_Y + PANEL_Y_PADDING)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
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
            self.env = Environment::new_rand(&self.env.params);
        }
    }

    /// Run and display the next step of the simulation
    pub fn run_next_step(&mut self) {
        self.env.advance_step();
    }

    /// Save the full current environment to a file
    fn save_environment(&self, filename : String) {
        let json_file_res = File::create(&filename);
        let mut json_file : File;
        match json_file_res {
            Err(e) => {
                println!("Error: could not load file {}. Error {e}", &filename);
                return;
            },
            Ok(f) => json_file = f,
        }

        json_file.write(self.env.to_json().as_bytes()).expect("Error writing environment to file!");
    }

    /// Load the full environment and creatures from json file
    fn load_environmnt(&mut self, filename : &str) {
        let load_opts : JsonEnvLoadParams = JsonEnvLoadParams {
            load_all : false,
            load_parameters : self.load_opts.load_params,
            load_creatures : self.load_opts.load_creatures,
            load_food : self.load_opts.load_food,
            load_walls : self.load_opts.load_walls,
        };

        self.env.load_from_json(filename, &load_opts);
    }

    /// Update the simulation env board
    fn update_sim_display(&self) {

        // Draw background
        draw_texture_ex(&self.background_texture, 0.0, 0.0, WHITE, self.background_options.clone());

        // For each simulation space on the board, update with proper piece
        for x in 0..self.env.params.env_x_size {
            for y in 0..self.env.params.env_y_size {
                match self.env.positions[x][y] {
                    SpaceStates::CreatureSpace(id) => {
                        let c_id = self.env.get_creature_idx_from_id(id).unwrap(); 
                        let creature : &Creature = &self.env.creatures[c_id];
                        self.draw_creature_square(x, y, creature.orientation, creature.color);
                    }
                    SpaceStates::FoodSpace => self.draw_food_space(x, y),
                    SpaceStates::WallSpace => self.draw_wall_space(x, y),
                    SpaceStates::FightSpace(_ttl) => self.draw_fight_space(x, y), 
                    SpaceStates::BlankSpace => (),
                }
            }
        }
    }

    /// Set the default "skin" (UI style) for macroquad
    fn set_default_skin(&mut self) {
        // Define style of the stats panel first
        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(0, 0,0, 255))
            .font_size(14)
            .build();
        let button_style = root_ui()
            .style_builder()
            .color(Color {r: 0.5, g: 0.5, b: 0.5, a: 1.0})
            .color_hovered(Color {r: 0.7, g: 0.7, b: 0.7, a: 1.0})
            .background_margin(RectOffset::new(40.0, 40.0, 5.0, 5.0))
            .font_size(16)
            .build();
        self.default_skin = Skin {
            label_style : label_style,
            button_style : button_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&self.default_skin);
    }

    /// Update the statistics panel
    fn update_stats_panel(&mut self) {

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
            stat_txt = format!("{:22} {:<12}", "NUM KILLS:", self.env.num_kills);
            ui.label(None, &stat_txt); 
            stat_txt = format!("{:22} {:<12}", "NUM NATURAL DEATHS:", self.env.num_natural_deaths);
            ui.label(None, &stat_txt); 

            // Get info on the space the mouse is hovering over
            ui.label(None, "");
            ui.label(None, "SPACE INFO:\n");
            let (mouse_x, mouse_y) = mouse_position();
            let env_x = (mouse_x / self.grid_x_size) as usize;
            let env_y = (mouse_y / self.grid_y_size) as usize;

            // If mouse is in environment, display info about the hovered space
            if env_x < self.env.params.env_x_size && env_y < self.env.params.env_y_size {
                let space_type = self.env.positions[env_x][env_y];
                ui.label(None, format!(" Space X:{} Y:{}    {:?}", env_x, env_y, space_type).as_str());
                match space_type {
                    SpaceStates::CreatureSpace(c_id) => {
                        let c_idx = self.env.get_creature_idx_from_id(c_id).unwrap();
                        let creature = &self.env.creatures[c_idx];
                        ui.label(None, format!("  Creature ID:      {}", creature.id).as_str());
                        ui.label(None, format!("  Age:              {}", creature.age).as_str());
                        ui.label(None, format!("  Energy:           {}", creature.energy).as_str());
                        ui.label(None, format!("  Color (r, g, b):  {}, {}, {}", creature.color.red, creature.color.green, creature.color.blue).as_str());
                        ui.label(None, format!("  Last Action:      {:?}", creature.last_action).as_str());
                        ui.label(None, format!("  Orientation:      {:?}", creature.orientation).as_str());
                        ui.label(None, format!("  Vision (r,g,b, dist): {}, {}, {}, {}", 
                            creature.vision_state.color.red,
                            creature.vision_state.color.green,
                            creature.vision_state.color.blue,
                            creature.vision_state.dist,
                            ).as_str());
                    },
                    _ => {},
                }
            }
            // Otherwise, display a key that shows what each space is
            else {
                ui.label(None, "GREEN       => Food space");
                ui.label(None, "WHITE       => Wall space");
                ui.label(None, "LIGHT RED   => Fight space (creature was killed here)");
                ui.label(None, "(ANY OTHER) => Creature space! (color can change w/ mutations)");
            }


            // Handle clicking to draw new space
            if is_mouse_button_down(MouseButton::Left) {
                if env_x < self.env.params.env_x_size && env_y < self.env.params.env_y_size {
                    let pos = Position {x : env_x, y : env_y};
                    match self.current_draw_space_type {
                        None => (),
                        Some(SpaceStates::FoodSpace) => self.env.add_food_space(pos),
                        Some(SpaceStates::WallSpace) => self.env.add_wall_space(pos),
                        Some(SpaceStates::BlankSpace) => self.env.add_blank_space(pos),
                        _ => (),
                    }

                }
            }

        });
        
    }

    /// Update the control panel on the right side of the window
    fn update_right_control_panel(&mut self) {
        root_ui().window(hash!(), vec2(self.control1_panel_x_pos, self.control1_panel_y_pos), vec2(CONTROL1_PANEL_WIDTH, CONTROL1_PANEL_HEIGHT), |ui| {
            ui.label(None, "CONTROL");

            // CONTROL PANEL
            if ui.button(None, "START/STOP") {
                self.state = match self.state {
                    SimState::RUNNING => SimState::STOPPED,
                    SimState::STOPPED => SimState::RUNNING,
                    SimState::FASTFORWARD => SimState::STOPPED,
                }
            }

            // Text box that gets step to jump to
            ui.input_text(hash!(), "Step to Jump to", &mut self.step_to_jump_to_str);
            let res = self.step_to_jump_to_str.parse::<usize>();
            match res {
                Err(_e) => self.step_to_jump_to = 0,
                Ok(step_val) => self.step_to_jump_to = step_val,
            }

            // Jump to a particular step button
            if ui.button(None, "JUMP TO STEP") {
                // If target step is reasonable, then enter fast forward mode
                if self.step_to_jump_to > self.env.time_step && self.step_to_jump_to > 0 && self.step_to_jump_to < 1000000 {
                    self.state = SimState::FASTFORWARD;
                }
            }

            let chosen_option = ui.combo_box(hash!(), "Space to Draw", &["None", "Food", "Wall", "Blank"], None);
            match chosen_option {
                0 => self.current_draw_space_type = None,
                1 => self.current_draw_space_type = Some(SpaceStates::FoodSpace),
                2 => self.current_draw_space_type = Some(SpaceStates::WallSpace),
                3 => self.current_draw_space_type = Some(SpaceStates::BlankSpace),
                _ => self.current_draw_space_type = None, 
            }
        });


    }

    /// Create/update the control panel in the UI
    fn update_bottom_control_panel(&mut self) {
        let text_height_px : f32 = 22.0;

        // Define the content of the control panel
        root_ui().window(hash!(), vec2(self.control_panel_x_pos, self.control_panel_y_pos), vec2(CONTROL2_PANEL_WIDTH, CONTROL2_PANEL_HEIGHT), |ui| {
            ui.label(Vec2{x: 5., y: 5.}, "CONTROL PANEL"); 
            ui.label(None, ""); 

            // Check boxes for specifying what to load - note I can't figure out the positioning of these...no idea how this is working, but it looks OK
            ui.same_line(0.);
            ui.checkbox(hash!(), "Load Parameters", &mut self.load_opts.load_params);
            ui.same_line(0.);
            ui.checkbox(hash!(), "Load Creatures", &mut self.load_opts.load_creatures);
            ui.same_line(0.);
            ui.checkbox(hash!(), "Load Walls", &mut self.load_opts.load_walls);
            ui.same_line(0.);
            ui.checkbox(hash!(), "Load Food", &mut self.load_opts.load_food);

            // Button to save the current environment as a json file
            if ui.button(Vec2{x : 0., y: text_height_px * 3.0}, "SAVE ENVIRONMENT") {
                self.save_environment(self.params.save_load_filename.clone());
            }
            // Button to save the current environment as a json file
            if ui.button(Vec2{x : 200.0, y: text_height_px * 3.0}, "LOAD ENVIRONMENT") {
                let temp_filename = self.params.save_load_filename.clone();
                self.load_environmnt(temp_filename.as_str());
            }

            // Text box that gets file name to load/save
            widgets::InputText::new(hash!())
                .position(Vec2{x: self.control_panel_x_pos + 5., y: self.control_panel_y_pos + text_height_px * 1.4})
                .label("Filename")
                .size(Vec2 { x: CONTROL2_PANEL_WIDTH / 1.5, y: text_height_px })
                .ui(ui, &mut self.params.save_load_filename);


            // ui.pop_skin();
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
        clear_background(BLACK);

        // Set style
        self.set_default_skin();

        // Update the main board
        self.update_sim_display();

        // Update statistics on the side
        self.update_stats_panel(); 

        // Update the control panel on the right side
        self.update_right_control_panel();

        // Update the simulation start parameters panel
        self.update_sim_param_panel();

        // Update the control panel below the environment display
        self.update_bottom_control_panel();
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
        let orientation_line_color = Color {r:0.8, g:0.8, b:0.8, a:1.0};

        match orientation {
            CreatureOrientation::Up => draw_line(center_x, center_y, center_x, center_y - y_gridsize_div_2, ORIENTATION_LINE_THICKNESS, orientation_line_color),
            CreatureOrientation::Down => draw_line(center_x, center_y, center_x, center_y + y_gridsize_div_2, ORIENTATION_LINE_THICKNESS, orientation_line_color),
            CreatureOrientation::Left => draw_line(center_x, center_y, center_x - x_gridsize_div_2, center_y, ORIENTATION_LINE_THICKNESS, orientation_line_color),
            CreatureOrientation::Right => draw_line(center_x, center_y, center_x + x_gridsize_div_2, center_y, ORIENTATION_LINE_THICKNESS, orientation_line_color),
        }
    }

    /// Draw a single food space on the screen
    fn draw_food_space(&self, x_pos : usize, y_pos : usize) {
        let food_color = Color {r: (FOOD_SPACE_COLOR[0] as f32) / 255.0, g: (FOOD_SPACE_COLOR[1] as f32) / 255.0, b : (FOOD_SPACE_COLOR[2] as f32) / 255.0 , a: 1.0};
        draw_rectangle((x_pos as f32) * self.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, food_color);
    }

    /// Draw a wall space on the screen
    fn draw_wall_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, WHITE);
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
        temp_params.env_y_size = self.params.env_y_size.parse::<usize>().expect("Error parsing env_y_size");
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
        if steps_to_go >= NUM_STEPS_PER_CALL {
            res = self.env.run_n_steps(NUM_STEPS_PER_CALL);
        } else {
            res = self.env.run_n_steps(steps_to_go);
        }

        // If we couldn't run the sim, just stop
        match res {
            Err(_e) => self.state = SimState::STOPPED,
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

    /// Main function to call from main loop that will update the display using the full interactive GUI
    /// This GUI displays the state of the environment as well as the full suite of controls
    /// This is in contrast to the display only mode which just simply displays the environment
    pub fn main_loop_interactive_mode(&mut self) {

        // If we're in fast forward mode, then simply run through this as fast as possible without updating display
        if self.state == SimState::FASTFORWARD {
            self.update_ff_mode();
        }

        // Update display every time through
        self.update_display();

        // Update current time
        let cur_time = get_time();

        // Decide whether we should run the next sim step
        if (self.state == SimState::RUNNING) && (cur_time - self.last_sim_update > MACROQUAD_FRAME_TIME_S) {
            self.run_next_step();
            self.last_sim_update = get_time();
        }

    }

}


