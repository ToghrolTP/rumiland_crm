# SCSS Structure Documentation

## Overview

This project uses a modular SCSS architecture for maintainable and scalable styles. The CSS is organized using the **7-1 pattern** (adapted).

## Directory Structure

```
scss/
├── styles.scss              # Main entry point (imports everything)
├── abstracts/
│   ├── _variables.scss      # Color, spacing, typography variables
│   └── _mixins.scss         # Reusable SCSS mixins
├── base/
│   ├── _reset.scss          # CSS reset
│   ├── _typography.scss     # Base typography styles
│   └── _animations.scss     # Keyframe animations
├── layout/
│   ├── _navbar.scss         # Navigation bar
│   ├── _container.scss      # Page containers and headers
│   └── _footer.scss         # Footer
├── components/
│   ├── _buttons.scss        # Button styles and variants
│   ├── _cards.scss          # Card component
│   ├── _tables.scss         # Table styles
│   ├── _forms.scss          # Form inputs and validation
│   ├── _flash-messages.scss # Flash/toast messages
│   ├── _badges.scss         # Badge components
│   ├── _alerts.scss         # Alert messages
│   ├── _spinners.scss       # Loading spinners
│   ├── _empty-states.scss   # Empty state components
│   ├── _details.scss        # Detail view layouts
│   └── _datepicker.scss     # Persian datepicker overrides
└── utilities/
    ├── _spacing.scss        # Margin/padding utilities
    ├── _text.scss           # Text styling utilities
    └── _responsive.scss     # Media queries and responsive styles
```

## Getting Started

### Installation

Install the required dependencies:

```bash
npm install
```

### Development

Watch for changes and auto-compile SCSS:

```bash
npm run dev
# or
npm run watch:css
```

### Production Build

Compile SCSS to compressed CSS:

```bash
npm run build:css
```

This will generate `static/css/styles.css` from `static/scss/styles.scss`.

## Architecture Principles

### 1. **Abstracts** (`abstracts/`)
- Contains variables, mixins, and functions
- No actual CSS output
- Must be imported first

### 2. **Base** (`base/`)
- Low-level styles (reset, typography)
- Global HTML element styles
- Animations and keyframes

### 3. **Layout** (`layout/`)
- Major layout components
- Page structure (navbar, footer, containers)

### 4. **Components** (`components/`)
- Reusable UI components
- Self-contained, modular pieces
- Most development happens here

### 5. **Utilities** (`utilities/`)
- Helper classes
- Responsive breakpoints
- Should be imported last

## Using Variables

All color, spacing, and typography variables are defined in `abstracts/_variables.scss`:

```scss
// Example usage
.my-component {
  background-color: $bg-primary;
  padding: $space-md;
  color: $text-primary;
  border-radius: $radius-lg;
}
```

## Using Mixins

Common patterns are available as mixins in `abstracts/_mixins.scss`:

```scss
// Button variant
.btn-custom {
  @include button-variant($accent-primary, #98971a);
}

// Alert variant
.alert-custom {
  @include alert-variant(rgba(255, 0, 0, 0.1), #ff0000);
}

// Responsive
.my-element {
  padding: $space-lg;
  
  @include mobile {
    padding: $space-sm;
  }
}
```

## Adding New Components

1. Create a new file in the appropriate directory:
   ```
   static/scss/components/_new-component.scss
   ```

2. Import it in `styles.scss`:
   ```scss
   @import 'components/new-component';
   ```

3. Use SCSS features for better organization:
   ```scss
   .component {
     property: value;
     
     &__element {
       property: value;
     }
     
     &--modifier {
       property: value;
     }
     
     &:hover {
       property: value;
     }
   }
   ```

## Naming Conventions

- Use **kebab-case** for class names: `.my-component`
- Use **BEM** for complex components: `.block__element--modifier`
- Prefix utility classes clearly: `.mb-2`, `.text-muted`
- Keep specificity low

## Best Practices

1. **Use variables** instead of hardcoded values
2. **Use mixins** for repeated patterns
3. **Nest selectors** logically (max 3-4 levels)
4. **Keep components isolated** and reusable
5. **Use descriptive names** for clarity
6. **Comment complex logic** when needed
7. **Mobile-first** approach with responsive mixins

## Variable Reference

### Colors
- `$bg-primary`, `$bg-secondary`, `$bg-tertiary`, `$bg-card`
- `$text-primary`, `$text-secondary`, `$text-muted`
- `$accent-primary`, `$accent-secondary`, `$accent-danger`, `$accent-warning`, `$accent-success`
- `$border-color`, `$border-light`

### Spacing
- `$space-xs` (0.25rem)
- `$space-sm` (0.5rem)
- `$space-md` (1rem)
- `$space-lg` (1.5rem)
- `$space-xl` (2rem)
- `$space-2xl` (3rem)

### Border Radius
- `$radius-sm`, `$radius-md`, `$radius-lg`, `$radius-full`

### Transitions
- `$transition-fast`, `$transition-base`, `$transition-slow`

### Shadows
- `$shadow-sm`, `$shadow-md`, `$shadow-lg`

## Build Output

- **Source**: `static/scss/styles.scss`
- **Output**: `static/css/styles.css`
- **Format**: Compressed (production) or Expanded (development)

## Troubleshooting

### SCSS not compiling?
- Ensure Node.js is installed: `node --version`
- Install dependencies: `npm install`
- Check file paths in imports

### Changes not reflecting?
- Make sure watch mode is running: `npm run dev`
- Clear browser cache
- Check console for compilation errors

### Import order issues?
- Variables and mixins must be imported before use
- Follow the order in `styles.scss`

## Migration from CSS

The old `static/css/styles.css` has been refactored into this modular SCSS structure with:
- ✅ Same visual output (1:1 equivalent)
- ✅ Better organization
- ✅ Reusable variables and mixins
- ✅ Easier maintenance
- ✅ Nested selectors for clarity

## Contributing

When adding new styles:
1. Choose the appropriate directory
2. Create a partial file (prefix with `_`)
3. Import in `styles.scss`
4. Use existing variables and mixins
5. Follow the established patterns
6. Test responsive behavior

---

**Note**: Always compile SCSS before deploying. The browser reads the compiled CSS, not the SCSS files.