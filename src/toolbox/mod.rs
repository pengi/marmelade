use crate::{
    runner::Runner,
    filesys::{
        hfs::HfsImage,
        rsrc::Rsrc
    }
};

pub struct Toolbox {
    runner: Runner
}

impl Toolbox {
    pub fn new(img: &HfsImage, rsrc: &Rsrc) -> std::io::Result<Toolbox> {
        Ok(Toolbox {
            runner: Runner::new(img, rsrc)?
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.runner.run()
    }
}