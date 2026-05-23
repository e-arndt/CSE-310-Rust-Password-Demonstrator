const passwordInput = document.getElementById("passwordInput");
const analyzeButton = document.getElementById("analyzeButton");
const resultBox = document.getElementById("resultBox");

const strongCharsetPattern = /^[A-Za-z0-9!@#$%^&*()\-_=\+\[\]{};:,.<>?\/|]+$/;

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