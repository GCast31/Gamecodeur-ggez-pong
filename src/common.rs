/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗                ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ ██████╗ ███╗   ██╗
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔═══██╗████╗  ██║
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ██║     ██║   ██║██╔████╔██║██╔████╔██║██║   ██║██╔██╗ ██║
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██║   ██║██║╚██╗██║
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║╚██████╔╝██║ ╚████║
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝                ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
                                                                                                                                                  
 */

pub struct VelocityRange {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Default for VelocityRange {
    fn default() -> Self {
        Self { 
            min_x: 0., 
            max_x: 0., 
            min_y: 0., 
            max_y: 0. 
        }
    }
}

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

pub struct Limits 
{
    pub min_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_x: Option<f32>,
    pub max_y: Option<f32>,
}

impl Default for Limits {
    fn default() -> Self {
        Limits { 
            min_x: None, 
            min_y: None, 
            max_x: None, 
            max_y: None 
        }
    }
}

impl Limits {
    
    pub fn comp(pos_actual: f32, pos_limit: Option<f32>) -> Result<f32, bool> {
        if let Some(max) = pos_limit {
            if pos_actual > max {
                return Ok(max);
            }
        }
        return Err(false);        
    }

    pub fn is_out_min_x(&self, x: f32) -> Result<f32, bool> {
        Limits::comp(x * -1., self.min_x)
    }

    pub fn is_out_min_y(&self, y: f32) -> Result<f32, bool> {
        Limits::comp(y * -1., self.min_y)
    }

    pub fn is_out_max_x(&self, x: f32) -> Result<f32, bool> {
        Limits::comp(x, self.max_x)
    }

    pub fn is_out_max_y(&self, y: f32) -> Result<f32, bool> {
        Limits::comp(y, self.max_y)
    }


}