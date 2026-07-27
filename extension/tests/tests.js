import init, { test_encrypt_roundtrip } from "./pkg/ghost.js";

// # Test:
// ## Identity 1:
// Identity "Public_key: kKub05heQ+Be+rhxAxM1eRU6PxQk4UZo3ialCarhEgI=,
// Private_key: 9n26Q2EnqsZqnnaBPvHVHRluxPgLGp4jS6u6EF5ZaLU="

// ## Identity 2:
// Identity "Public_key: RnIBlYG2eEeBvQKQQBJLu+Ax1dOC9nAJeYvhZmixjmA=,
// Private_key: oCvBIuKVGBI9F3kAR3uNJVmOAaoG5wKp/attMBlUGTY="


async function main() {
  await init();

  document.getElementById("test-btn").addEventListener("click", async () => {
    const my_private_key = "9n26Q2EnqsZqnnaBPvHVHRluxPgLGp4jS6u6EF5ZaLU=";
    const their_public_key = "RnIBlYG2eEeBvQKQQBJLu+Ax1dOC9nAJeYvhZmixjmA=";

    const message = document.getElementById("message").value;
    
    const result = test_encrypt_roundtrip(my_private_key, their_public_key, message);

    document.getElementById("test-result").textContent = result;
  })
}

main();