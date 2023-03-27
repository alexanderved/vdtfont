pub struct Buffer<T: ocl::OclPrm> {
    inner: ocl::Buffer<T>,
    len: usize,
    cap: usize,
}

impl<T: ocl::OclPrm> Buffer<T> {
    pub fn new(queue: ocl::Queue) -> anyhow::Result<Self> {
        let inner = ocl::Buffer::<T>::builder().queue(queue).len(1).build()?;

        Ok(Self { inner, len: 0, cap: 1 })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_raw(&self) -> &ocl::Buffer<T> {
        &self.inner
    }

    pub fn read(&mut self, dst: &mut [T]) -> anyhow::Result<()> {
        if self.len() == 0 {
            anyhow::bail!("Buffer is empty");
        }

        self.inner.read(dst).enq()?;

        Ok(())
    }

    pub fn write(&mut self, src: &[T]) -> anyhow::Result<()> {
        if src.len() > self.cap {
            self.realloc(src.len())?;
        }

        self.inner.write(src).enq()?;
        self.len = src.len();

        Ok(())
    }

    pub fn first(&self) -> anyhow::Result<T> {
        if self.len() == 0 {
            anyhow::bail!("Buffer is empty");
        }

        let mut first = [T::default()];
        self.inner.cmd().read(&mut first as &mut [T]).len(1).enq()?;

        Ok(first[0])
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        if self.len() == 0 {
            anyhow::bail!("Buffer is empty");
        }

        let cleared_data = vec![T::default(); self.len()];
        self.inner.write(&cleared_data).enq().unwrap();

        Ok(())
    }

    fn realloc(&mut self, cap: usize) -> anyhow::Result<()> {
        if cap > self.cap {
            self.inner = ocl::Buffer::<T>::builder()
                .queue(self.inner.default_queue().unwrap().clone())
                .len(cap)
                .build()?;
        }

        Ok(())
    }
}
