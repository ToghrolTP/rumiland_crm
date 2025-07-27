// Form submission debugger
// Add this to any form page to debug submission issues

(function () {
  "use strict";

  document.addEventListener("DOMContentLoaded", function () {
    const forms = document.querySelectorAll("form");

    forms.forEach((form, index) => {
      console.log(`📋 Found form ${index + 1}:`, form.action, form.method);

      // Log form data before submission
      form.addEventListener("submit", function (e) {
        console.log("🚀 Form submitting...");
        const formData = new FormData(form);

        console.log("📦 Form data:");
        for (let [key, value] of formData.entries()) {
          console.log(`  ${key}: ${value}`);
        }

        // Check for disabled inputs
        const disabledInputs = form.querySelectorAll(
          "input:disabled, textarea:disabled, select:disabled",
        );
        if (disabledInputs.length > 0) {
          console.warn(
            "⚠️ WARNING: Found disabled inputs that won't be submitted:",
            disabledInputs,
          );
        }

        // Check for readonly inputs
        const readonlyInputs = form.querySelectorAll(
          "input[readonly], textarea[readonly]",
        );
        if (readonlyInputs.length > 0) {
          console.log(
            "📝 Readonly inputs (will still submit):",
            readonlyInputs,
          );
        }
      });
    });
  });
})();
