// Global error handler for better UX
(function() {
    'use strict';
    
    // Network error detection
    let isOnline = navigator.onLine;
    
    // Check network status
    function checkNetworkStatus() {
        if (!navigator.onLine && isOnline) {
            // Just went offline
            isOnline = false;
            showNetworkError();
        } else if (navigator.onLine && !isOnline) {
            // Just came online
            isOnline = true;
            hideNetworkError();
        }
    }
    
    // Show network error banner
    function showNetworkError() {
        let banner = document.getElementById('network-error-banner');
        
        if (!banner) {
            banner = document.createElement('div');
            banner.id = 'network-error-banner';
            banner.className = 'network-error-banner';
            banner.innerHTML = `
                <div class="network-error-content">
                    <span class="network-error-icon">📡</span>
                    <span class="network-error-text">اتصال اینترنت قطع شده است</span>
                    <button type="button" class="network-retry-btn" onclick="location.reload()">
                        تلاش مجدد
                    </button>
                </div>
            `;
            document.body.appendChild(banner);
        }
        
        banner.classList.add('show');
    }
    
    // Hide network error banner
    function hideNetworkError() {
        const banner = document.getElementById('network-error-banner');
        if (banner) {
            banner.classList.remove('show');
            setTimeout(() => {
                location.reload();
            }, 1000);
        }
    }
    
    // Listen for network changes
    window.addEventListener('online', checkNetworkStatus);
    window.addEventListener('offline', checkNetworkStatus);
    
    // Handle fetch errors globally
    const originalFetch = window.fetch;
    window.fetch = function(...args) {
        return originalFetch.apply(this, args)
            .catch(error => {
                if (!navigator.onLine) {
                    showNetworkError();
                }
                throw error;
            });
    };
    
    // Handle form validation errors
    document.addEventListener('DOMContentLoaded', function() {
        // Add custom validation messages to all forms
        const forms = document.querySelectorAll('form');
        
        forms.forEach(form => {
            // Add novalidate to handle validation ourselves
            form.setAttribute('novalidate', '');
            
            form.addEventListener('submit', function(e) {
                if (!validateForm(form)) {
                    e.preventDefault();
                    showFormErrors(form);
                }
            });
            
            // Real-time validation
            const inputs = form.querySelectorAll('input, textarea, select');
            inputs.forEach(input => {
                input.addEventListener('blur', function() {
                    validateField(input);
                });
                
                input.addEventListener('input', function() {
                    clearFieldError(input);
                });
            });
        });
    });
    
    // Validate entire form
    function validateForm(form) {
        let isValid = true;
        const inputs = form.querySelectorAll('input[required], textarea[required], select[required]');
        
        inputs.forEach(input => {
            if (!validateField(input)) {
                isValid = false;
            }
        });
        
        return isValid;
    }
    
    // Validate individual field
    function validateField(field) {
        let isValid = true;
        let errorMessage = '';
        
        // Clear previous error
        clearFieldError(field);
        
        // Required field check
        if (field.hasAttribute('required') && !field.value.trim()) {
            errorMessage = 'این فیلد الزامی است';
            isValid = false;
        }
        
        // Type-specific validation
        else if (field.type === 'email' && field.value) {
            const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
            if (!emailRegex.test(field.value)) {
                errorMessage = 'آدرس ایمیل معتبر نیست';
                isValid = false;
            }
        }
        
        else if (field.type === 'tel' && field.value) {
            const phoneRegex = /^0[0-9]{10}$/;
            const cleanPhone = field.value.replace(/\D/g, '');
            if (!phoneRegex.test(cleanPhone)) {
                errorMessage = 'شماره تلفن معتبر نیست';
                isValid = false;
            }
        }
        
        else if (field.minLength && field.value.length < field.minLength) {
            errorMessage = `حداقل ${field.minLength} کاراکتر وارد کنید`;
            isValid = false;
        }
        
        else if (field.pattern && field.value) {
            const pattern = new RegExp(field.pattern);
            if (!pattern.test(field.value)) {
                errorMessage = field.getAttribute('data-error') || 'فرمت وارد شده صحیح نیست';
                isValid = false;
            }
        }
        
        if (!isValid) {
            showFieldError(field, errorMessage);
        }
        
        return isValid;
    }
    
    // Show error for a field
    function showFieldError(field, message) {
        field.classList.add('input-error');
        
        let errorEl = field.parentElement.querySelector('.field-error-message');
        if (!errorEl) {
            errorEl = document.createElement('p');
            errorEl.className = 'field-error-message';
            field.parentElement.appendChild(errorEl);
        }
        
        errorEl.textContent = message;
    }
    
    // Clear error for a field
    function clearFieldError(field) {
        field.classList.remove('input-error');
        
        const errorEl = field.parentElement.querySelector('.field-error-message');
        if (errorEl) {
            errorEl.remove();
        }
    }
    
    // Show form errors summary
    function showFormErrors(form) {
        const errors = form.querySelectorAll('.input-error');
        if (errors.length > 0) {
            errors[0].focus();
            errors[0].scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    }
    
    // Handle AJAX errors
    window.addEventListener('unhandledrejection', function(event) {
        console.error('Unhandled promise rejection:', event.reason);
        
        if (event.reason && event.reason.message) {
            if (event.reason.message.includes('Failed to fetch')) {
                showNetworkError();
            }
        }
    });
    
})();

// CSS for network error banner
const style = document.createElement('style');
style.textContent = `
.network-error-banner {
    position: fixed;
    top: -100px;
    left: 0;
    right: 0;
    background-color: var(--accent-danger);
    color: white;
    padding: var(--space-md);
    text-align: center;
    z-index: 9999;
    transition: top 0.3s ease-out;
    box-shadow: var(--shadow-lg);
}

.network-error-banner.show {
    top: 0;
}

.network-error-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-md);
}

.network-error-icon {
    font-size: 1.25rem;
    animation: pulse 2s ease-in-out infinite;
}

.network-retry-btn {
    background-color: white;
    color: var(--accent-danger);
    border: none;
    padding: var(--space-xs) var(--space-md);
    border-radius: var(--radius-sm);
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
}

.network-retry-btn:hover {
    opacity: 0.9;
}
`;
document.head.appendChild(style);