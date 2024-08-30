for i in 0 to 10 {
  if i > 5 {
    break;
  };
  for j in 0 to 10 step 2 {
    print("i:" + str(i) + " j:" + str(j));
  }
}