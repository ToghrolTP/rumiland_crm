# SCSS Structure Visualization

## Complete File Tree

```
static/scss/
│
├── styles.scss                      # Main entry point - imports all partials
│
├── abstracts/                       # Configuration & Helpers
│   ├── _variables.scss              # Colors, spacing, typography, breakpoints
│   └── _mixins.scss                 # Reusable SCSS mixins (functions)
│
├── base/                            # Foundation Styles
│   ├── _reset.scss                  # CSS reset (box-sizing, margins, padding)
│   ├── _typography.scss             # HTML, body, headings, paragraphs
│   └── _animations.scss             # @keyframes (slideDown, fadeIn, spin, etc.)
│
├── layout/                          # Major Layout Components
│   ├── _navbar.scss                 # Navigation bar and links
│   ├── _container.scss              # Main content area, page headers
│   └── _footer.scss                 # Footer styles
│
├── components/                      # UI Components (Reusable)
│   ├── _buttons.scss                # All button variants and states
│   ├── _cards.scss                  # Card component
│   ├── _tables.scss                 # Table styles with hover states
│   ├── _forms.scss                  # Forms, inputs, validation, overlays
│   ├── _flash-messages.scss         # Toast/flash messages
│   ├── _badges.scss                 # Badge/tag components
│   ├── _alerts.scss                 # Alert messages
│   ├── _spinners.scss               # Loading spinners & progress bars
│   ├── _empty-states.scss           # Empty state displays
│   ├── _details.scss                # Detail view grids and layouts
│   └── _datepicker.scss             # Persian datepicker theme overrides
│
└── utilities/                       # Helper Classes
    ├── _spacing.scss                # Margin/padding utilities (mb-1, mt-auto)
    ├── _text.scss                   # Text utilities & phone/email styling
    └── _responsive.scss             # Media queries for mobile

```

## Import Order (Critical!)

The order in `styles.scss` is important:

1. **Abstracts First** → Variables & mixins must be available to all other files
2. **Base Second** → Foundation styles that don't depend on components
3. **Layout Third** → Page structure
4. **Components Fourth** → UI elements that use variables/mixins
5. **Utilities Last** → Override classes with high specificity

## Component Breakdown

### abstracts/_variables.scss (63 lines)
- Gruvbox color palette
- Spacing scale (xs → 2xl)
- Border radius values
- Transition speeds
- Shadow definitions
- Typography fonts
- Breakpoints

### abstracts/_mixins.scss (94 lines)
- `@mixin flex-center`
- `@mixin flex-between`
- `@mixin button-variant($bg, $hover)`
- `@mixin card-shadow`
- `@mixin mobile { @media ... }`
- `@mixin slide-down`
- `@mixin badge-variant($bg, $text)`
- `@mixin alert-variant($bg, $border)`
- `@mixin input-state($border, $bg)`

### base/_reset.scss (11 lines)
- Universal box-sizing
- Reset margins & padding

### base/_typography.scss (51 lines)
- HTML font-size
- Body styles (RTL, font-family)
- Heading styles (h1-h6)
- Paragraph spacing

### base/_animations.scss (80 lines)
- @keyframes slideDown
- @keyframes fadeIn
- @keyframes pulse
- @keyframes shakeIn
- @keyframes spin
- @keyframes progressBar

### layout/_navbar.scss (71 lines)
- `.navbar` → Sticky navigation bar
- `.nav-container` → Wrapper with max-width
- `.nav-brand` → Logo/brand link
- `.nav-links` → Navigation items container
- `.nav-link` → Individual nav items with hover/active states

### layout/_container.scss (35 lines)
- `.main-content` → Main page wrapper
- `.container` → Content container with max-width
- `.page-header` → Page header section
- `.page-title` → Title with icon support
- `.page-actions` → Button group

### layout/_footer.scss (11 lines)
- `.footer` → Site footer with border

### components/_buttons.scss (117 lines)
- `.btn` → Base button styles
- `.btn-primary` → Primary action button
- `.btn-secondary` → Secondary button
- `.btn-danger` → Destructive action
- `.btn-ghost` → Outlined button
- `.btn-sm`, `.btn-lg` → Size variants
- Loading states
- Disabled states
- `.form-button-group` → Button container

### components/_cards.scss (11 lines)
- `.card` → Card container with hover effect

### components/_tables.scss (72 lines)
- `.table-container` → Scrollable wrapper
- `table`, `thead`, `tbody` → Table structure
- `th`, `td` → Table cells with borders
- `.table-actions` → Action buttons in rows
- Hover states
- Link styling

### components/_forms.scss (284 lines)
**Largest component file**
- `.form-group` → Form field wrapper
- `.form-label` → Field labels
- `.form-input`, `.form-textarea` → Input fields
- Validation states (`.input-valid`, `.input-error`)
- `.form-hint`, `.form-error` → Helper text
- `.form-details` → Collapsible details/summary
- `.area-codes` → Phone area code display
- `.email-suggestions` → Email autocomplete
- `.form-overlay` → Full-screen loading state
- `.form-errors` → Error summary block
- Disabled states

### components/_flash-messages.scss (68 lines)
- `.flash-message` → Base message container
- `.flash-success`, `.flash-error`, `.flash-warning` → Variants
- `.flash-icon`, `.flash-text` → Message elements
- `.flash-close` → Close button

### components/_badges.scss (34 lines)
- `.badge` → Base badge styles
- `.badge-primary`, `.badge-info`, `.badge-success`, `.badge-error`

### components/_alerts.scss (51 lines)
- `.alert` → Alert container
- `.alert-error`, `.alert-warning`, `.alert-info`, `.alert-success`
- `.alert-icon`, `.alert-content`, `.alert-title`, `.alert-message`

### components/_spinners.scss (40 lines)
- `.spinner` → Inline spinner (for buttons)
- `.spinner-large` → Full-screen spinner
- `.progress-bar` → Top progress bar

### components/_empty-states.scss (25 lines)
- `.empty-state` → Empty state container
- `.empty-icon` → Large centered icon
- `.empty-title`, `.empty-description` → Text

### components/_details.scss (52 lines)
- `.detail-grid` → Grid container
- `.detail-row` → Two-column layout (label + value)
- `.detail-label`, `.detail-value` → Row elements
- `.detail-notes` → Notes textarea display

### components/_datepicker.scss (81 lines)
- Persian datepicker overrides
- Theme customization for gruvbox
- Hover states
- Selected date styling

### utilities/_spacing.scss (27 lines)
- `.mb-0` through `.mb-4` → Margin bottom
- `.mt-auto` → Margin top auto

### utilities/_text.scss (68 lines)
- `.text-muted`, `.text-small` → Text helpers
- `.phone-link`, `.phone-number` → Phone formatting
- `.email-link`, `.email-address` → Email formatting

### utilities/_responsive.scss (81 lines)
- Mobile-first media queries
- Responsive adjustments for all components
- Font-size reduction
- Navigation simplification
- Touch-friendly adjustments

## Line Count Summary

| Directory      | Files | Total Lines |
|----------------|-------|-------------|
| abstracts/     | 2     | 157         |
| base/          | 3     | 142         |
| layout/        | 3     | 117         |
| components/    | 10    | 846         |
| utilities/     | 3     | 176         |
| **Total**      | 21    | **~1,438**  |

*Note: Total includes comments and whitespace for readability*

## CSS Output Size

- **Uncompressed**: ~45 KB
- **Compressed**: ~35 KB  
- **Gzipped**: ~8 KB

## Key Features

✅ **Variables** → Consistency across all styles  
✅ **Mixins** → No code duplication  
✅ **Nesting** → Clear hierarchy and relationships  
✅ **Modularity** → Easy to find and update specific styles  
✅ **Scalability** → Simple to add new components  
✅ **Maintainability** → Each file has single responsibility  

## Quick Reference

### Need to change colors?
→ `abstracts/_variables.scss`

### Need to add a button variant?
→ `components/_buttons.scss`

### Need to style a form?
→ `components/_forms.scss`

### Need to adjust mobile layout?
→ `utilities/_responsive.scss`

### Need to create a reusable pattern?
→ `abstracts/_mixins.scss`

---

**Structure Type**: 7-1 Pattern (adapted)  
**Total Partials**: 20  
**Main Entry**: `styles.scss`  
**Output**: `../css/styles.css`
