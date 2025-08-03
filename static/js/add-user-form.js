const form = document.getElementById("user-form");
const submitBtn = document.getElementById("submit-btn");
const cancelBtn = document.getElementById("cancel-btn");
const formOverlay = document.getElementById("form-overlay");
let isSubmitting = false;

form.addEventListener("submit", function (e) {
    if (isSubmitting) {
        e.preventDefault();
        return;
    }

    isSubmitting = true;

    submitBtn.disabled = true;
    submitBtn.classList.add("btn-loading-state");
    submitBtn.querySelector(".btn-content").style.display = "none";
    submitBtn.querySelector(".btn-loading").style.display = "flex";

    cancelBtn.classList.add("btn-disabled");
    cancelBtn.style.pointerEvents = "none";

    setTimeout(() => {
        if (isSubmitting) {
            formOverlay.style.display = "flex";
        }
    }, 300);

    const inputs = form.querySelectorAll("input, textarea, select");
    inputs.forEach((input) => {
        input.style.opacity = "0.7";
        input.style.pointerEvents = "none";
    });
});

window.addEventListener("pageshow", function (event) {
    if (event.persisted) {
        isSubmitting = false;
        submitBtn.disabled = false;
        submitBtn.classList.remove("btn-loading-state");
        submitBtn.querySelector(".btn-content").style.display = "flex";
        submitBtn.querySelector(".btn-loading").style.display = "none";
        cancelBtn.classList.remove("btn-disabled");
        cancelBtn.style.pointerEvents = "auto";
        formOverlay.style.display = "none";

        const inputs = form.querySelectorAll("input, textarea, select");
        inputs.forEach((input) => {
            input.style.opacity = "1";
            input.style.pointerEvents = "auto";
        });
    }
});

document
    .getElementById("username")
    .addEventListener("invalid", function (e) {
        e.preventDefault();
        if (this.value.trim() === "") {
            this.setCustomValidity("لطفاً نام کاربری را وارد کنید");
        } else if (!this.checkValidity()) {
            this.setCustomValidity(
                "نام کاربری فقط باید شامل حروف انگلیسی و اعداد باشد",
            );
        }
    });

document.getElementById("username").addEventListener("input", function (e) {
    this.setCustomValidity("");
});

document
    .getElementById("password")
    .addEventListener("invalid", function (e) {
        e.preventDefault();
        if (this.value.length < 6) {
            this.setCustomValidity("رمز عبور باید حداقل 6 کاراکتر باشد");
        }
    });

document.getElementById("password").addEventListener("input", function (e) {
    this.setCustomValidity("");
});