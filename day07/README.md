This one is pretty rough. I didn't realize that there were constants mixed in with the identifiers, and the type system I set up made it difficult to recover from that. So instead of doing anything elegant, I just added the numbers 0-99 to the constant list so that they would get looked up regularly, as if there was a gate named "1" with the value `1`. Not very elegant or scalable, but it worked.

Also, I don't reset and re-solve the puzzle for part 2. It requires manually changing the value.

The `Gate::Alias` type is cool though, that means you don't have to modify the file to solve it because of the `foo -> a` entry lurking in the file.