const passwordInput = document.getElementById("passwordInput");
const analyzeButton = document.getElementById("analyzeButton");
const resultBox = document.getElementById("resultBox");

const weakPasswordInput = document.getElementById("weakPasswordInput");
const weakDemoButton = document.getElementById("weakDemoButton");
const weakResultBox = document.getElementById("weakResultBox");

const weakPasswordPattern = /^[a-z]+$/;

const strongCharsetPattern = /^[A-Za-z0-9!@#$%^&*()\-_=\+\[\]{};:,.<>?\/|]+$/;



weakDemoButton.addEventListener("click", async () => {
  const password = weakPasswordInput.value.trim();

  if (!password) {
    weakResultBox.textContent = "Please enter a weak password first.";
    return;
  }

  if (password.length > 5) {
    weakResultBox.textContent = "Weak demo passwords must be 5 characters or fewer.";
    return;
  }

  if (!weakPasswordPattern.test(password)) {
    weakResultBox.textContent = "Weak demo passwords must use lowercase letters a-z only.";
    return;
  }

  weakDemoButton.disabled = true;
  weakResultBox.textContent = "Running weak brute-force demo...";

  try {
    const response = await fetch("http://127.0.0.1:3000/api/weak-demo", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ password }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      weakResultBox.textContent = `Rust server rejected input: ${errorText}`;
      return;
    }

    const data = await response.json();

    weakResultBox.innerHTML = `
      <strong>Status:</strong> ${data.status}<br>
      <strong>Password Found:</strong> ${data.password_found}<br>
      <strong>Attempts:</strong> ${data.attempts}<br>
      <strong>Elapsed Time:</strong> ${data.elapsed_seconds.toFixed(4)} seconds<br>
      <strong>Local CPU Rate:</strong> ${data.guesses_per_second} guesses/sec<br><br>

      <strong>Target Hash:</strong><br>
      ${data.target_hash}<br><br>

      <strong>Matched Hash:</strong><br>
      ${data.matched_hash}
    `;
  } catch (error) {
    weakResultBox.innerHTML = `
      <strong>Connection Failed</strong><br>
      Make sure the Rust server is running on port 3000.
    `;

    console.error(error);
  } finally {
    weakDemoButton.disabled = false;
  }
});


analyzeButton.addEventListener("click", async () => {
  const password = passwordInput.value.trim();

  if (!password) {
    resultBox.textContent = "Please enter a password first.";
    return;
  }

  if (password.length > 8) {
    resultBox.textContent = "Strong estimator passwords must be 8 characters or fewer.";
    return;
  }

  if (!strongCharsetPattern.test(password)) {
    resultBox.textContent = "Password contains unsupported characters.";
    return;
  }

  resultBox.textContent = "Sending password to Rust estimator...";

  try {
    const response = await fetch("http://127.0.0.1:3000/api/estimate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ password }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      resultBox.textContent = `Rust server rejected input: ${errorText}`;
      return;
    }

    const data = await response.json();

    resultBox.innerHTML = `
      <strong>Rust API Status:</strong> ${data.status}<br><br>

      <strong>Password Length:</strong> ${data.password_length}<br>
      <strong>Charset:</strong> ${data.charset_label}<br>
      <strong>Charset Size:</strong> ${data.charset_size}<br>
      <strong>Local Rate:</strong> ${data.local_rate} guesses/sec<br><br>

      <strong>Estimated Attempts:</strong> ${data.estimated_attempts}<br>
      <strong>Estimated Time:</strong> ${data.estimated_time}
    `;
  } catch (error) {
    resultBox.innerHTML = `
      <strong>Connection Failed</strong><br>
      Make sure the Rust server is running on port 3000.
    `;

    console.error(error);
  }
});