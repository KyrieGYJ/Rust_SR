pub struct ISPPipeBuffer<T: Default + Clone> {
    data: Vec<T>,
    width: u32,
    height: u32,
    channel: u8,
}

impl<T: Default + Clone> ISPPipeBuffer<T> {
    pub fn new(width: u32, height: u32, channel: u8) -> ISPPipeBuffer<T> {
        let len = (width * height * channel as u32) as usize;
        let data: Vec<T> = vec![T::default(); len];
        ISPPipeBuffer {
            data,
            width,
            height,
            channel
        }
    }
}

pub trait ISPProcessor<T: Default + Clone> {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T>;
}

// BLC
struct BlackLevelCompensation {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for BlackLevelCompensation {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// DPC
struct DeadPixelCorrection {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for DeadPixelCorrection {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// LSC
struct LensShadingCorrection {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for LensShadingCorrection {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}


// AAF(deleted)
struct AntiAliasingFiltering {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for AntiAliasingFiltering {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// RDD
struct RawDomainDenoising {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for RawDomainDenoising {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// AWB
struct AutoWhiteBalanceGainControl {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for AutoWhiteBalanceGainControl {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// Demosaic
struct CFAInterpolation {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for CFAInterpolation {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// CCM
struct ColorCorrectionMatrix {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for ColorCorrectionMatrix {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// DRC
struct DynamicRangeCompression {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for DynamicRangeCompression {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// GC
struct GammaCorrection {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for GammaCorrection {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// CSC (rgb to yuv)
struct ColorSpaceConversion {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for ColorSpaceConversion {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// CE
struct ColorEnhancement {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for ColorEnhancement {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// EE
struct EdgeEnhancement {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for EdgeEnhancement {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}


// NF for yuv
struct NoiseFilterForChroma {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for NoiseFilterForChroma {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// FCS
struct FalseColorSuppresion {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for FalseColorSuppresion {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

// TM
struct ToneMapping {
    // params...
}

impl<T: Default + Clone> ISPProcessor<T> for ToneMapping {
    fn process(&self, input_buffer: ISPPipeBuffer<T>) -> ISPPipeBuffer<T> {
        let width = input_buffer.width;
        let height = input_buffer.height;
        let channel = input_buffer.channel;
        ISPPipeBuffer::new(width, height, channel)
    }
}

pub fn basic_pipe(sensor_input: Vec<i32>) {
    // let processor_list: Vec<dyn ISPProcessor<i32>> = Vec::new();
}