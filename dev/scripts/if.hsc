let bool = 1;
let msg = if bool { "true" } else { "false" };
print("bool: " + msg);

let end = if if bool { 0 } else { 1 } { 0 } else { 1 };

for i in 0 to if end { 6 } else { 11 } {
  print(i)
}