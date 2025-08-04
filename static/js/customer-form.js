// Wrap everything in IIFE to avoid conflicts
(function () {
  "use strict";

  // Wait for DOM to be fully loaded
  document.addEventListener("DOMContentLoaded", function () {
    initializeForm();
  });

  function initializeForm() {
    // Get all required elements with error checking
    const phoneInput = document.getElementById("phone_number");
    const phoneError = document.getElementById("phone-error");
    const phoneStatus = document.getElementById("phone-status");
    const emailInput = document.getElementById("email");
    const emailError = document.getElementById("email-error");
    const emailStatus = document.getElementById("email-status");
    const emailSuggestions = document.getElementById("email-suggestions");
    const emailSuggestionBtn = document.getElementById("email-suggestion");
    const form = document.querySelector("form");
    const submitBtn = document.getElementById("submit-btn");
    const cancelBtn = document.getElementById("cancel-btn");
    const formOverlay = document.getElementById("form-overlay");

    // Check if all required elements exist
    if (!phoneInput || !emailInput || !form) {
      console.error("Required form elements not found");
      return;
    }

    let isSubmitting = false;

    // Phone validation functions
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
        const isThreeDigitArea = majorCities.some((code) =>
          value.startsWith(code),
        );

        if (isThreeDigitArea) {
          return (
            value.slice(0, 3) +
            " " +
            value.slice(3, 7) +
            " " +
            value.slice(7, 11)
          );
        } else {
          return (
            value.slice(0, 4) +
            " " +
            value.slice(4, 7) +
            " " +
            value.slice(7, 11)
          );
        }
      }
      return value;
    }

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
        if (phoneError) phoneError.style.display = "none";
        if (phoneStatus) {
          phoneStatus.textContent = "";
          phoneStatus.className = "form-label-badge";
        }
      } else if (isValid) {
        phoneInput.classList.add("input-valid");
        phoneInput.classList.remove("input-error");
        if (phoneError) phoneError.style.display = "none";
        if (phoneStatus) {
          phoneStatus.textContent = `✓ ${phoneType}`;
          phoneStatus.className = "form-label-badge badge-success";
        }
      } else {
        phoneInput.classList.add("input-error");
        phoneInput.classList.remove("input-valid");
        if (phoneError) phoneError.style.display = "block";
        if (phoneStatus) {
          phoneStatus.textContent = "✗ نامعتبر";
          phoneStatus.className = "form-label-badge badge-error";
        }
      }
    }

    // Email validation functions
    function isValidEmail(email) {
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      return emailRegex.test(email);
    }

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

      if (!domain.includes(".") && domain.length > 3) {
        for (const commonDomain of commonDomains) {
          if (commonDomain.startsWith(domain.toLowerCase())) {
            return `${localPart}@${commonDomain}`;
          }
        }
      }

      return null;
    }

    // Initialize phone validation for existing value
    let initialPhoneValue = phoneInput.value.replace(/\D/g, "");
    if (initialPhoneValue.length === 11) {
      phoneInput.value = formatPhoneNumber(initialPhoneValue);
      validatePhone();
    }

    // Phone input event listener
    phoneInput.addEventListener("input", function (e) {
      let value = e.target.value.replace(/\D/g, "");
      let formatted = "";

      const phoneType = getPhoneType(value);

      if (phoneType === "mobile" || phoneType === "mobile-no-zero") {
        if (phoneType === "mobile-no-zero") {
          value = "0" + value;
        }

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
        const isThreeDigitArea = majorCities.some((code) =>
          value.startsWith(code),
        );

        if (isThreeDigitArea) {
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

    // Initialize email validation for existing value
    if (emailInput.value) {
      emailInput.dispatchEvent(new Event("input"));
    }

    // Email input event listener
    emailInput.addEventListener("input", function (e) {
      // Convert Persian/Arabic characters
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

      const email = e.target.value.trim();

      if (email.length === 0) {
        emailInput.classList.remove("input-valid", "input-error");
        if (emailError) emailError.style.display = "none";
        if (emailStatus) {
          emailStatus.textContent = "";
          emailStatus.className = "form-label-badge";
        }
        if (emailSuggestions) emailSuggestions.style.display = "none";
        return;
      }

      const isValid = isValidEmail(email);

      if (isValid) {
        emailInput.classList.add("input-valid");
        emailInput.classList.remove("input-error");
        if (emailError) emailError.style.display = "none";
        if (emailStatus) {
          emailStatus.textContent = "✓ معتبر";
          emailStatus.className = "form-label-badge badge-success";
        }
        if (emailSuggestions) emailSuggestions.style.display = "none";
      } else {
        emailInput.classList.add("input-error");
        emailInput.classList.remove("input-valid");
        if (emailStatus) {
          emailStatus.textContent = "✗ نامعتبر";
          emailStatus.className = "form-label-badge badge-error";
        }

        if (email.length > 5) {
          if (emailError) emailError.style.display = "block";

          const suggestion = suggestEmail(email);
          if (suggestion && emailSuggestionBtn && emailSuggestions) {
            emailSuggestionBtn.textContent = suggestion;
            emailSuggestions.style.display = "block";
          } else if (emailSuggestions) {
            emailSuggestions.style.display = "none";
          }
        }
      }
    });

    // Email suggestion click handler
    if (emailSuggestionBtn) {
      emailSuggestionBtn.addEventListener("click", function () {
        emailInput.value = this.textContent;
        emailInput.dispatchEvent(new Event("input"));
        emailInput.focus();
      });
    }

    // Custom validation messages
    phoneInput.addEventListener("invalid", function (e) {
      e.preventDefault();
      this.setCustomValidity("لطفاً یک شماره تلفن معتبر وارد کنید");
    });

    phoneInput.addEventListener("input", function (e) {
      this.setCustomValidity("");
    });

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

    // Form submission handling
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
      if (submitBtn) {
        submitBtn.disabled = true;
        submitBtn.classList.add("btn-loading-state");
        const btnContent = submitBtn.querySelector(".btn-content");
        const btnLoading = submitBtn.querySelector(".btn-loading");
        if (btnContent) btnContent.style.display = "none";
        if (btnLoading) btnLoading.style.display = "flex";
      }

      // Disable cancel button
      if (cancelBtn) {
        cancelBtn.classList.add("btn-disabled");
        cancelBtn.style.pointerEvents = "none";
      }

      // Show overlay after a slight delay
      setTimeout(() => {
        if (isSubmitting && formOverlay) {
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
        input.style.pointerEvents = "none";
      });
    });

    // Reset form state if user navigates back
    window.addEventListener("pageshow", function (event) {
      if (event.persisted) {
        // Page was loaded from cache (back button)
        isSubmitting = false;

        if (submitBtn) {
          submitBtn.disabled = false;
          submitBtn.classList.remove("btn-loading-state");
          const btnContent = submitBtn.querySelector(".btn-content");
          const btnLoading = submitBtn.querySelector(".btn-loading");
          if (btnContent) btnContent.style.display = "flex";
          if (btnLoading) btnLoading.style.display = "none";
        }

        if (cancelBtn) {
          cancelBtn.classList.remove("btn-disabled");
          cancelBtn.style.pointerEvents = "auto";
        }

        if (formOverlay) formOverlay.style.display = "none";

        document
          .querySelectorAll(".progress-bar")
          .forEach((bar) => bar.remove());

        const inputs = form.querySelectorAll("input, textarea, select");
        inputs.forEach((input) => {
          input.style.opacity = "1";
          input.style.pointerEvents = "auto";
        });
      }
    });
  }
})();
