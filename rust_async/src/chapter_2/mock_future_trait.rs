trait SimpleFuture {
    type Output;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;

    // fn has_data_to_read(&self) -> bool;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct Socket {}
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    // fn has_data_to_read(&self) -> bool {
    //     return false;
    // }

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            return Poll::Ready(());
        } else {
            return Poll::Pending;
        }
    }
}

pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => {
                    self.first.take();
                }
                // We couldn't yet complete the first future.
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }

        self.second.poll(wake)
    }
}

trait Future {
    type Ouput;

    // Pin allows us to create futures that are immovable
    // Immovable objects can store pointers beween fields. Pinning is necssary to enable async/await

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Ouput>;
}
