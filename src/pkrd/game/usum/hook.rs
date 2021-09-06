use super::{frame, reader};
use crate::pkrd::{
    display,
    game::{HookableProcess, HookedProcess, Reader, SupportedTitle},
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonUSUM {
    title: SupportedTitle,
}

impl HookedProcess for PokemonUSUM {
    fn run_hook(&self, heap: Reader, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game_reader = reader::PokemonUSUMReader::new(heap);
        frame::run(game_reader, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonUSUM {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self> {
        Box::new(Self { title })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        Self::patch_present_framebuffer(
            process,
            pkrd_handle,
            0x30000000,
            0x34a8d84,
            0x279bb4,
            0x630000,
            0x27ab38,
        )
    }
}