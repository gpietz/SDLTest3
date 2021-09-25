#![warn(clippy::pedantic)]

use rg3d_sound:: {
    engine::SoundEngine,
    source::{
        generic::GenericSourceBuilder,
        SoundSource,
        Status
    },
    context::SoundContext,
    buffer::{
        DataSource,
        SoundBufferResource
    },
};

use std::sync::{Arc, Mutex};
use rg3d_sound::futures::executor::block_on;
use rg3d_sound::pool::Handle;

pub struct SoundApi {
    engine: Arc<Mutex<SoundEngine>>,
    context: SoundContext,
    stream: Option<Handle<SoundSource>>,
}

impl SoundApi {
    pub fn new() -> Self {
        Self {
            engine: SoundEngine::new(),
            context: SoundContext::new(),
            stream: Option::None
        }
    }

    fn create_streaming_buffer(file_name: &str) -> SoundBufferResource {
        SoundBufferResource::new_streaming(block_on(DataSource::from_file(file_name)).unwrap()).unwrap()
    }

    fn create_source(sound_buffer: SoundBufferResource, status: Status) -> SoundSource {
        GenericSourceBuilder::new()
            .with_buffer(sound_buffer)
            .with_status(status)
            .build_source()
            .unwrap()
    }

    fn create_playing_source(sound_buffer: SoundBufferResource) -> SoundSource {
        SoundApi::create_source(sound_buffer, Status::Playing)
    }

    pub fn play_streamed(&mut self, file_name: &str) {
        self.add_context();
        self.stop_streaming();
        let buffer = SoundApi::create_streaming_buffer(file_name);
        let source = SoundApi::create_playing_source(buffer);
        let mut state = self.context.state();
        self.stream = Option::Some(state.add_source(source));
    }

    fn add_context(&mut self) {
        let mut engine = self.engine.lock().unwrap();
        if engine.contexts().len() > 0 {
            return;
        }
        engine.add_context(self.context.clone());
    }

    pub fn stop_streaming(&mut self) {
        if self.stream.is_some() {
            let source_handle = self.stream.unwrap();
            let mut source_state = self.context.state();
            let source = source_state.source_mut(source_handle);
            source.stop().unwrap();
            self.stream = Option::None;
        }
    }

    pub fn set_stream_looped(&mut self, looped: bool) -> bool {
        if self.stream.is_some() {
            let source_handle = self.stream.unwrap();
            let mut source_state = self.context.state();
            let source = source_state.source_mut(source_handle);
            source.set_looping(looped);
            return true;
        }
        false
    }

    pub fn is_stream_playing_looped(&mut self) -> bool {
        if self.stream.is_some() {
            let source_handle = self.stream.unwrap();
            let mut source_state = self.context.state();
            let source = source_state.source_mut(source_handle);
            return source.is_looping();
        }
        false
    }
}
