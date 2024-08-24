use std::sync::mpsc;
use std::sync::{Arc,Mutex};


pub struct TwoWayChannel<T: Send + Sync> {
    my_sender: Mutex<mpsc::Sender<T>>,
    other_receiver: Mutex<mpsc::Receiver<T>>,
}

impl<T: Send + Sync> TwoWayChannel<T> { 
    fn new(my_sender: Mutex<mpsc::Sender<T>>, other_receiver: Mutex<mpsc::Receiver<T>>) -> TwoWayChannel<T> {
        TwoWayChannel {
            my_sender: my_sender,
            other_receiver: other_receiver,
        }
    }

    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.my_sender.lock().unwrap().send(t)?;
        Ok(())
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        let item = self.other_receiver.lock().unwrap().recv()?;
        Ok(item)
    }

}


pub fn channel<T: Send + Sync>() -> (Arc<TwoWayChannel<T>>,Arc<TwoWayChannel<T>>) {
    let (send_1, recv_1) = mpsc::channel::<T>();
    let (send_2, recv_2) = mpsc::channel::<T>();

    let channel_1 = Arc::new(TwoWayChannel::new(Mutex::new(send_1), Mutex::new(recv_2)));
    let channel_2 = Arc::new(TwoWayChannel::new(Mutex::new(send_2), Mutex::new(recv_1)));
    (channel_1, channel_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn send_down_channel() {
        let (channel_1, channel_2) = channel::<i32>();

        channel_1.send(5).unwrap();

        assert_eq!(channel_2.recv().unwrap(), 5);
        
    }

    #[test]
    fn send_down_channel_multithreaded() {
        let (channel_1, channel_2) = channel::<i32>();

        std::thread::spawn(move || {
            Arc::clone(&channel_1).send(2763).unwrap();
        });

        assert_eq!(channel_2.recv().unwrap(), 2763);
    }
}
