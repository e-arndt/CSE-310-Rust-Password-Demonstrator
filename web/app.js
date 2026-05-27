// ================================
// DOM element references
// Connects JavaScript logic to the web page inputs, buttons, and result boxes.
// ================================

const passwordInput = document.getElementById("passwordInput");
const analyzeButton = document.getElementById("analyzeButton");
const resultBox = document.getElementById("resultBox");

const weakPasswordInput = document.getElementById("weakPasswordInput");
const weakDemoButton = document.getElementById("weakDemoButton");
const weakResultBox = document.getElementById("weakResultBox");

const weakPasswordPattern = /^[a-z]+$/;

const moderatePasswordInput = document.getElementById("moderatePasswordInput");
const moderateDemoButton = document.getElementById("moderateDemoButton");
const moderateResultBox = document.getElementById("moderateResultBox");

const moderatePasswordPattern = /^[A-Za-z0-9]+$/;

const strongCharsetPattern = /^[A-Za-z0-9!@#$%^&*()\-_=\+\[\]{};:,.<>?\/|]+$/;

const currentYear = document.getElementById("currentYear");

// ================================
// Footer year setup
// Updates the footer year automatically when the page loads.
// ================================

if (currentYear) {
  currentYear.textContent = new Date().getFullYear();
}

// ================================
// Demo run state
// Tracks whether a brute-force demo is already running.
// ================================

let demoRunning = false;

// ================================
// Demo control locking
// Enables or disables both real brute-force demo controls at the same time.
// ================================

function setDemoControlsDisabled(disabled) {
  weakPasswordInput.disabled = disabled;
  weakDemoButton.disabled = disabled;

  moderatePasswordInput.disabled = disabled;
  moderateDemoButton.disabled = disabled;
}

// ================================
// Hash chunk formatter
// Splits long SHA-256 hashes into shorter display chunks.
// ================================

function splitHashIntoChunks(hash, chunkSize = 16) {
  if (!hash) {
    return [];
  }

  const chunks = [];

  for (let i = 0; i < hash.length; i += chunkSize) {
    chunks.push(hash.slice(i, i + chunkSize));
  }

  return chunks;
}

// ================================
// Hash comparison renderer
// Builds the side-by-side hash comparison display for demo results.
// ================================

function renderHashComparison(targetHash, matchedHash) {
  const targetChunks = splitHashIntoChunks(targetHash);
  const matchedChunks = splitHashIntoChunks(matchedHash);

  return `
    <div class="hash-comparison">
      <div class="hash-column">
        <strong>Target Hash:</strong>
        <div class="hash-chunks">
          ${targetChunks.map((chunk) => `<span>${chunk}</span>`).join("")}
        </div>
      </div>

      <div class="hash-column">
        <strong>Matched Hash:</strong>
        <div class="hash-chunks">
          ${matchedChunks.map((chunk) => `<span>${chunk}</span>`).join("")}
        </div>
      </div>
    </div>
  `;
}

// ================================
// Weak password demo handler
// Validates lowercase-only input, calls the Rust weak-demo API, and renders results.
// ================================

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

  if (demoRunning) {
  weakResultBox.textContent = "Another brute-force demo is already running. Please wait for it to finish.";
  return;
}

  demoRunning = true;
  setDemoControlsDisabled(true);
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
      <strong>Charset Size:</strong> ${data.charset_size}<br>
      <strong>Attempts:</strong> ${data.attempts}<br>
      <strong>Elapsed Time:</strong> ${data.elapsed_seconds.toFixed(4)} seconds<br>
      <strong>Local CPU Rate:</strong> ${data.guesses_per_second} guesses/sec

      ${renderHashComparison(data.target_hash, data.matched_hash)}
    `;

  } catch (error) {
    weakResultBox.innerHTML = `
      <strong>Connection Failed</strong><br>
      Make sure the Rust server is running on port 3000.
    `;

    console.error(error);
  } finally {
    demoRunning = false;
    setDemoControlsDisabled(false);
  }
});


// ================================
// Moderate password demo handler
// Validates mixed-case letter/digit input, calls the Rust moderate-demo API, and renders results.
// ================================

moderateDemoButton.addEventListener("click", async () => {
  const password = moderatePasswordInput.value.trim();

  if (!password) {
    moderateResultBox.textContent = "Please enter a moderate password first.";
    return;
  }

  if (password.length > 5) {
    moderateResultBox.textContent = "Moderate demo passwords must be 5 characters or fewer.";
    return;
  }

  if (!moderatePasswordPattern.test(password)) {
    moderateResultBox.textContent =
      "Moderate demo passwords must use letters A-Z, a-z, and digits 0-9 only.";
    return;
  }

  if (demoRunning) {
  moderateResultBox.textContent = "Another brute-force demo is already running. Please wait for it to finish.";
  return;
}

  demoRunning = true;
  setDemoControlsDisabled(true);
  moderateResultBox.textContent = "Running moderate brute-force demo...";

  try {
    const response = await fetch("http://127.0.0.1:3000/api/moderate-demo", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ password }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      moderateResultBox.textContent = `Rust server rejected input: ${errorText}`;
      return;
    }

    const data = await response.json();

    moderateResultBox.innerHTML = `
      <strong>Status:</strong> ${data.status}<br>
      <strong>Password Found:</strong> ${data.password_found}<br>
      <strong>Charset Size:</strong> ${data.charset_size}<br>
      <strong>Attempts:</strong> ${data.attempts}<br>
      <strong>Elapsed Time:</strong> ${data.elapsed_seconds.toFixed(4)} seconds<br>
      <strong>Local CPU Rate:</strong> ${data.guesses_per_second} guesses/sec

      ${renderHashComparison(data.target_hash, data.matched_hash)}
    `;
  } catch (error) {
    moderateResultBox.innerHTML = `
      <strong>Connection Failed</strong><br>
      Make sure the Rust server is running on port 3000.
    `;

    console.error(error);
  } finally {
    demoRunning = false;
    setDemoControlsDisabled(false);
  }
});


// ================================
// Strong password estimator handler
// Validates stronger password input, calls the Rust estimate API, and renders estimate results.
// ================================

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
      <strong>Local Rate:</strong> ${data.local_rate} guesses/sec<br>
      <strong>Rate Source:</strong> ${data.rate_source}<br><br>

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