# SCSS Quick Reference Card

## 🚀 Getting Started

```bash
# Install dependencies
npm install

# Build CSS once
npm run build:css

# Watch for changes (development)
npm run dev
```

## 📁 File Structure

```
scss/
├── styles.scss              # Main entry (import all files here)
├── abstracts/               # Variables & mixins
├── base/                    # Reset, typography, animations
├── layout/                  # Navbar, container, footer
├── components/              # Buttons, forms, cards, etc.
└── utilities/               # Spacing, text, responsive
```

## 🎨 Variables Reference

### Colors
```scss
// Backgrounds
$bg-primary: #1d2021;
$bg-secondary: #282828;
$bg-tertiary: #3c3836;
$bg-card: #32302f;

// Text
$text-primary: #fbf1c7;
$text-secondary: #ebdbb2;
$text-muted: #a89984;

// Accents
$accent-primary: #b8bb26;     // Green
$accent-secondary: #83a598;   // Blue
$accent-danger: #fb4934;      // Red
$accent-warning: #fabd2f;     // Yellow
$accent-success: #b8bb26;     // Green

// Borders
$border-color: #504945;
$border-light: #3c3836;
```

### Spacing
```scss
$space-xs: 0.25rem;    // 4px
$space-sm: 0.5rem;     // 8px
$space-md: 1rem;       // 16px
$space-lg: 1.5rem;     // 24px
$space-xl: 2rem;       // 32px
$space-2xl: 3rem;      // 48px
```

### Border Radius
```scss
$radius-sm: 0.375rem;
$radius-md: 0.5rem;
$radius-lg: 0.75rem;
$radius-full: 9999px;
```

### Transitions
```scss
$transition-fast: 150ms ease;
$transition-base: 200ms ease;
$transition-slow: 300ms ease;
```

### Shadows
```scss
$shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
$shadow-md: 0 4px 6px rgba(0, 0, 0, 0.4);
$shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.5);
```

## 🔧 Useful Mixins

### Flexbox
```scss
.my-element {
  @include flex-center;      // Center items
  @include flex-between;     // Space between
  @include flex-start;       // Align start
}
```

### Button Variant
```scss
.btn-custom {
  @include button-variant($bg-color, $hover-color);
}
```

### Badge Variant
```scss
.badge-custom {
  @include badge-variant($bg-color, $text-color);
}
```

### Alert Variant
```scss
.alert-custom {
  @include alert-variant($bg-color, $border-color);
}
```

### Input State
```scss
.input-custom {
  @include input-state($border-color, $bg-color);
}
```

### Responsive
```scss
.my-element {
  padding: $space-lg;
  
  @include mobile {
    padding: $space-sm;
  }
}
```

### Animations
```scss
.animated {
  @include slide-down;
  @include fade-in;
}
```

## 📝 Common Patterns

### Basic Component
```scss
.my-component {
  background-color: $bg-card;
  padding: $space-lg;
  border-radius: $radius-md;
  transition: all $transition-base;

  &:hover {
    box-shadow: $shadow-md;
  }

  &__element {
    color: $text-primary;
  }

  &--modifier {
    background-color: $bg-secondary;
  }
}
```

### Nested Selectors
```scss
.parent {
  property: value;

  .child {
    property: value;
  }

  &:hover {
    property: value;
  }

  &.active {
    property: value;
  }
}
```

### Button with States
```scss
.btn-custom {
  padding: $space-sm $space-lg;
  background-color: $accent-primary;
  color: $bg-primary;
  border-radius: $radius-md;
  transition: all $transition-base;

  &:hover {
    background-color: darken($accent-primary, 10%);
    transform: translateY(-1px);
  }

  &:active {
    transform: translateY(0);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}
```

## 🎯 Adding New Components

### 1. Create Partial
```bash
touch static/scss/components/_my-component.scss
```

### 2. Write Styles
```scss
// _my-component.scss
.my-component {
  background: $bg-card;
  padding: $space-md;
  
  &__title {
    color: $text-primary;
  }
}
```

### 3. Import in styles.scss
```scss
// Add to styles.scss
@import 'components/my-component';
```

### 4. Compile
```bash
npm run build:css
```

## 🐛 Troubleshooting

### CSS not updating?
```bash
# Stop watch, rebuild, restart
Ctrl+C
npm run build:css
npm run dev
```

### Compilation error?
- Check syntax (semicolons, braces)
- Verify variable names
- Ensure imports are in order
- Check file paths

### Variables not working?
- Make sure abstracts are imported first
- Check variable is defined in _variables.scss
- Look for typos

## 💡 Best Practices

✅ **DO**
- Use variables for colors and spacing
- Use mixins for repeated patterns
- Keep nesting 3-4 levels max
- Use meaningful class names
- Comment complex logic
- Follow BEM naming convention

❌ **DON'T**
- Hardcode colors or spacing
- Duplicate code (use mixins)
- Over-nest selectors
- Use !important unless absolutely necessary
- Mix tabs and spaces
- Leave dead code

## 📦 NPM Scripts

| Command | Description |
|---------|-------------|
| `npm install` | Install dependencies |
| `npm run build:css` | Compile SCSS to CSS (production) |
| `npm run watch:css` | Watch and auto-compile |
| `npm run dev` | Same as watch:css |

## 🎨 Color Usage Guide

```scss
// Backgrounds
background-color: $bg-primary;      // Main page background
background-color: $bg-secondary;    // Input backgrounds
background-color: $bg-card;         // Card backgrounds

// Text
color: $text-primary;               // Headings, important text
color: $text-secondary;             // Body text
color: $text-muted;                 // Labels, hints

// Accents
color: $accent-primary;             // Primary actions, success
color: $accent-secondary;           // Links, info
color: $accent-danger;              // Errors, delete actions
color: $accent-warning;             // Warnings
```

## 🔍 Finding Code

| Need to... | Look in... |
|------------|-----------|
| Change colors | `abstracts/_variables.scss` |
| Add button style | `components/_buttons.scss` |
| Modify forms | `components/_forms.scss` |
| Adjust tables | `components/_tables.scss` |
| Change navbar | `layout/_navbar.scss` |
| Add spacing utility | `utilities/_spacing.scss` |
| Mobile styles | `utilities/_responsive.scss` |

## 📚 File Size

- Source SCSS: ~1,400 lines (across 21 files)
- Compiled CSS: ~35 KB (compressed)
- Gzipped: ~8 KB

---

**Last Updated**: 2024  
**SCSS Version**: Dart Sass 1.69.5+  
**Pattern**: 7-1 Architecture