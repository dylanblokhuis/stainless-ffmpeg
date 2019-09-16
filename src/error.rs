macro_rules! check_result {
  ($condition: expr, $block: block) => {
    let errnum = $condition;
    if errnum < 0 {
      let mut data = [0i8; AV_ERROR_MAX_STRING_SIZE as usize];
      av_strerror(errnum, data.as_mut_ptr(), AV_ERROR_MAX_STRING_SIZE);
      $block;
      return Err(tools::to_string(data.as_ptr()));
    }
  };
  ($condition: expr) => {
    let errnum = $condition;
    if errnum < 0 {
      let mut data = [0i8; AV_ERROR_MAX_STRING_SIZE as usize];
      av_strerror(errnum, data.as_mut_ptr(), AV_ERROR_MAX_STRING_SIZE);
      return Err(tools::to_string(data.as_ptr()));
    }
  };
}
