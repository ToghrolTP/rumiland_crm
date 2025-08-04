const phoneInput = document.getElementById("phone_number");
const phoneError = document.getElementById("phone-error");
const phoneStatus = document.getElementById("phone-status");

function getPhoneType(digits) {
  if (digits.length === 11 && digits.startsWith("09")) {
    return "mobile";
  } else if (digits.length === 11 && digits.startsWith("0")) {
    return "landline";
  } else if (digits.length === 10 && digits.startsWith("9")) {
    return "mobile-no-zero";
  }
  return null;
}

window.addEventListener("DOMContentLoaded", function () {
  let value = phoneInput.value.replace(/\D/g, "");

  if (value.length === 11) {
    phoneInput.value = formatPhoneNumber(value);
    validatePhone();
  }
});

function formatPhoneNumber(value) {
  const phoneType = getPhoneType(value);

  if (phoneType === "mobile") {
    return (
      value.slice(0, 4) + " " + value.slice(4, 7) + " " + value.slice(7, 11)
    );
  } else if (phoneType === "landline") {
    const majorCities = [
      "021",
      "026",
      "031",
      "041",
      "051",
      "071",
      "061",
      "034",
    ];
    const isThreeDigitArea = majorCities.some((code) => value.startsWith(code));

    if (isThreeDigitArea) {
      return (
        value.slice(0, 3) + " " + value.slice(3, 7) + " " + value.slice(7, 11)
      );
    } else {
      return (
        value.slice(0, 4) + " " + value.slice(4, 7) + " " + value.slice(7, 11)
      );
    }
  }
  return value;
}

phoneInput.addEventListener("input", function (e) {
  let value = e.target.value.replace(/\D/g, "");
  let formatted = "";

  const phoneType = getPhoneType(value);

  if (phoneType === "mobile" || phoneType === "mobile-no-zero") {
    // Mobile format
    if (phoneType === "mobile-no-zero") {
      value = "0" + value;
    }

    if (value.length <= 4) {
      formatted = value;
    } else if (value.length <= 7) {
      formatted = value.slice(0, 4) + " " + value.slice(4);
    } else {
      formatted =
        value.slice(0, 4) + " " + value.slice(4, 7) + " " + value.slice(7, 11);
    }
  } else if (phoneType === "landline") {
    // Check if it's a major city with 3-digit area code
    const majorCities = [
      "021",
      "026",
      "031",
      "041",
      "051",
      "071",
      "061",
      "034",
    ];
    const isThreeDigitArea = majorCities.some((code) => value.startsWith(code));

    if (isThreeDigitArea) {
      // Format: 021 4455 6677
      if (value.length <= 3) {
        formatted = value;
      } else if (value.length <= 7) {
        formatted = value.slice(0, 3) + " " + value.slice(3);
      } else {
        formatted =
          value.slice(0, 3) +
          " " +
          value.slice(3, 7) +
          " " +
          value.slice(7, 11);
      }
    } else {
      // Format: 0241 333 4444
      if (value.length <= 4) {
        formatted = value;
      } else if (value.length <= 7) {
        formatted = value.slice(0, 4) + " " + value.slice(4);
      } else {
        formatted =
          value.slice(0, 4) +
          " " +
          value.slice(4, 7) +
          " " +
          value.slice(7, 11);
      }
    }
  } else {
    // Unknown format
    if (value.length <= 4) {
      formatted = value;
    } else if (value.length <= 7) {
      formatted = value.slice(0, 4) + " " + value.slice(4);
    } else {
      formatted =
        value.slice(0, 4) + " " + value.slice(4, 7) + " " + value.slice(7);
    }
  }

  e.target.value = formatted;
  validatePhone();
});

// Validate phone number
function validatePhone() {
  const value = phoneInput.value.replace(/\D/g, "");
  let isValid = false;
  let phoneType = "";

  if (value.length === 11) {
    if (value.startsWith("09")) {
      isValid = true;
      phoneType = "موبایل";
    } else if (value.startsWith("0")) {
      isValid = true;
      phoneType = "ثابت";
    }
  } else if (value.length === 10 && value.startsWith("9")) {
    isValid = true;
    phoneType = "موبایل";
  }

  if (value.length === 0) {
    phoneInput.classList.remove("input-valid", "input-error");
    phoneError.style.display = "none";
    phoneStatus.textContent = "";
    phoneStatus.className = "form-label-badge";
  } else if (isValid) {
    phoneInput.classList.add("input-valid");
    phoneInput.classList.remove("input-error");
    phoneError.style.display = "none";
    phoneStatus.textContent = `✓ ${phoneType}`;
    phoneStatus.className = "form-label-badge badge-success";
  } else {
    phoneInput.classList.add("input-error");
    phoneInput.classList.remove("input-valid");
    phoneError.style.display = "block";
    phoneStatus.textContent = "✗ نامعتبر";
    phoneStatus.className = "form-label-badge badge-error";
  }
}

// Prevent form submission if phone is invalid
document.querySelector("form").addEventListener("submit", function (e) {
  const value = phoneInput.value.replace(/\D/g, "");
  let isValid = false;

  if (value.length === 11 && value.startsWith("0")) {
    isValid = true;
  } else if (value.length === 10 && value.startsWith("9")) {
    // Add leading zero before submission
    phoneInput.value = "0" + value;
    isValid = true;
  }

  if (!isValid) {
    e.preventDefault();
    phoneInput.focus();
    validatePhone();
  }
});

// Custom validation message in Persian
phoneInput.addEventListener("invalid", function (e) {
  e.preventDefault();
  this.setCustomValidity("لطفاً یک شماره تلفن معتبر وارد کنید");
});

phoneInput.addEventListener("input", function (e) {
  this.setCustomValidity("");
});

// Email validation and suggestions
const emailInput = document.getElementById("email");
const emailError = document.getElementById("email-error");
const emailStatus = document.getElementById("email-status");
const emailSuggestions = document.getElementById("email-suggestions");
const emailSuggestionBtn = document.getElementById("email-suggestion");

// Common email domains
const commonDomains = [
  "gmail.com",
  "yahoo.com",
  "outlook.com",
  "hotmail.com",
  "mail.com",
  "protonmail.com",
  "icloud.com",
  "gmail.ir",
  "yahoo.ir",
  "chmail.ir",
];

// Validate initial email on load
window.addEventListener("DOMContentLoaded", function () {
  if (emailInput.value) {
    emailInput.dispatchEvent(new Event("input"));
  }
});

// Validate email format
function isValidEmail(email) {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// Check for common typos
function suggestEmail(email) {
  if (!email.includes("@")) return null;

  const [localPart, domain] = email.split("@");
  if (!domain) return null;

  const typoMap = {
    "gmial.com": "gmail.com",
    "gmai.com": "gmail.com",
    "gmil.com": "gmail.com",
    "gmail.co": "gmail.com",
    "gmail.con": "gmail.com",
    "gmail.ocm": "gmail.com",
    "yahoo.co": "yahoo.com",
    "yahoo.con": "yahoo.com",
    "yaho.com": "yahoo.com",
    "yahooo.com": "yahoo.com",
    "hotmial.com": "hotmail.com",
    "hotmai.com": "hotmail.com",
    "hotmil.com": "hotmail.com",
    "outlok.com": "outlook.com",
    "outloook.com": "outlook.com",
  };

  if (typoMap[domain.toLowerCase()]) {
    return `${localPart}@${typoMap[domain.toLowerCase()]}`;
  }

  if (!domain.includes(".") && domain.length > 3) {
    for (const commonDomain of commonDomains) {
      if (commonDomain.startsWith(domain.toLowerCase())) {
        return `${localPart}@${commonDomain}`;
      }
    }
  }

  return null;
}

// Email validation on input
emailInput.addEventListener("input", function (e) {
  const email = e.target.value.trim();

  if (email.length === 0) {
    emailInput.classList.remove("input-valid", "input-error");
    emailError.style.display = "none";
    emailStatus.textContent = "";
    emailStatus.className = "form-label-badge";
    emailSuggestions.style.display = "none";
    return;
  }

  const isValid = isValidEmail(email);

  if (isValid) {
    emailInput.classList.add("input-valid");
    emailInput.classList.remove("input-error");
    emailError.style.display = "none";
    emailStatus.textContent = "✓ معتبر";
    emailStatus.className = "form-label-badge badge-success";
    emailSuggestions.style.display = "none";
  } else {
    emailInput.classList.add("input-error");
    emailInput.classList.remove("input-valid");
    emailStatus.textContent = "✗ نامعتبر";
    emailStatus.className = "form-label-badge badge-error";

    if (email.length > 5) {
      emailError.style.display = "block";

      const suggestion = suggestEmail(email);
      if (suggestion) {
        emailSuggestionBtn.textContent = suggestion;
        emailSuggestions.style.display = "block";
      } else {
        emailSuggestions.style.display = "none";
      }
    }
  }
});

// Apply suggestion
emailSuggestionBtn.addEventListener("click", function () {
  emailInput.value = this.textContent;
  emailInput.dispatchEvent(new Event("input"));
  emailInput.focus();
});

// Custom validation messages
emailInput.addEventListener("invalid", function (e) {
  e.preventDefault();
  const email = this.value.trim();

  if (email.length === 0) {
    this.setCustomValidity("لطفاً آدرس ایمیل خود را وارد کنید");
  } else if (!email.includes("@")) {
    this.setCustomValidity("آدرس ایمیل باید شامل @ باشد");
  } else if (!email.includes(".")) {
    this.setCustomValidity(
      "آدرس ایمیل باید شامل نام دامنه باشد (مثلاً gmail.com)",
    );
  } else {
    this.setCustomValidity("لطفاً یک آدرس ایمیل معتبر وارد کنید");
  }
});

emailInput.addEventListener("input", function (e) {
  this.setCustomValidity("");
});

// Convert Persian/Arabic characters
emailInput.addEventListener("input", function (e) {
  const persianToEnglish = {
    "۰": "0",
    "۱": "1",
    "۲": "2",
    "۳": "3",
    "۴": "4",
    "۵": "5",
    "۶": "6",
    "۷": "7",
    "۸": "8",
    "۹": "9",
    "٠": "0",
    "١": "1",
    "٢": "2",
    "٣": "3",
    "٤": "4",
    "٥": "5",
    "٦": "6",
    "٧": "7",
    "٨": "8",
    "٩": "9",
    "@": "@",
    "＠": "@",
    "٪": "@",
  };

  let value = e.target.value;
  let hasChanges = false;

  for (const [persian, english] of Object.entries(persianToEnglish)) {
    if (value.includes(persian)) {
      value = value.replace(new RegExp(persian, "g"), english);
      hasChanges = true;
    }
  }

  if (hasChanges) {
    e.target.value = value;
  }
});

// Form submission handling
const form = document.querySelector("form");
const submitBtn = document.getElementById("submit-btn");
const cancelBtn = document.getElementById("cancel-btn");
const formOverlay = document.getElementById("form-overlay");
let isSubmitting = false;

// Handle form submission
form.addEventListener("submit", function (e) {
  // Prevent multiple submissions
  if (isSubmitting) {
    e.preventDefault();
    return;
  }

  // Check phone validation first
  const phoneValue = phoneInput.value.replace(/\D/g, "");
  let isPhoneValid = false;

  if (phoneValue.length === 11 && phoneValue.startsWith("0")) {
    isPhoneValid = true;
  } else if (phoneValue.length === 10 && phoneValue.startsWith("9")) {
    // Add leading zero before submission
    phoneInput.value = "0" + phoneValue;
    isPhoneValid = true;
  }

  if (!isPhoneValid) {
    e.preventDefault();
    phoneInput.focus();
    validatePhone();
    return;
  }

  // Check email validation
  const emailValue = emailInput.value.trim();
  if (!isValidEmail(emailValue)) {
    e.preventDefault();
    emailInput.focus();
    return;
  }

  // Mark as submitting
  isSubmitting = true;

  // Update button state
  submitBtn.disabled = true;
  submitBtn.classList.add("btn-loading-state");
  submitBtn.querySelector(".btn-content").style.display = "none";
  submitBtn.querySelector(".btn-loading").style.display = "flex";

  // Disable cancel button
  cancelBtn.classList.add("btn-disabled");
  cancelBtn.style.pointerEvents = "none";

  // Show overlay after a slight delay
  setTimeout(() => {
    if (isSubmitting) {
      formOverlay.style.display = "flex";

      // Add progress bar
      const progressBar = document.createElement("div");
      progressBar.className = "progress-bar";
      document.body.appendChild(progressBar);
    }
  }, 300);

  // Visual feedback for inputs without disabling
  const inputs = form.querySelectorAll("input, textarea, select");
  inputs.forEach((input) => {
    input.style.opacity = "0.7";
    input.style.pointerEvents = "none"; // Prevent interaction but keep data
  });
});

// Reset form state if user navigates back
window.addEventListener("pageshow", function (event) {
  if (event.persisted) {
    // Page was loaded from cache (back button)
    isSubmitting = false;
    submitBtn.disabled = false;
    submitBtn.classList.remove("btn-loading-state");
    submitBtn.querySelector(".btn-content").style.display = "flex";
    submitBtn.querySelector(".btn-loading").style.display = "none";
    cancelBtn.classList.remove("btn-disabled");
    cancelBtn.style.pointerEvents = "auto";
    formOverlay.style.display = "none";

    document.querySelectorAll(".progress-bar").forEach((bar) => bar.remove());

    const inputs = form.querySelectorAll("input, textarea, select");
    inputs.forEach((input) => {
      input.style.opacity = "1";
      input.style.pointerEvents = "auto";
    });
  }
});
