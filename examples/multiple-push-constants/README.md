# multiple-push-constants

Runs a compute shader to compute numbers multiplied by a multiplier, and with a bias added. This
demonstrates using two push constant variables in a shader.

## To Run

```
# Pass in a multiplier, a bias, and any numbers following.
RUST_LOG=multiple_push_constants cargo run --bin multiple_push_constants 2 4 3 295
```

## Example Output

```
[2020-04-25T11:15:33Z INFO  hello_compute] Steps: [10, 594]
```
