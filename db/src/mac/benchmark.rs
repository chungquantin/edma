macro_rules! profile {
 ($($token:tt)+) => {
  {
   let _instant = std::time::Instant::now();
   let _result = {
       $($token)+
   };
   println!("Elapse time: {:?}", _instant.elapsed());
   _result
  }
 }
}
