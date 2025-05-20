# Kumeo UX Guidelines

## Table of Contents

1. [Design Principles](#design-principles)
2. [Color Palette](#color-palette)
3. [Typography](#typography)
4. [UI Components](#ui-components)
5. [Interaction Patterns](#interaction-patterns)
6. [Responsiveness](#responsiveness)
7. [Accessibility](#accessibility)
8. [Voice and Tone](#voice-and-tone)

---

## Design Principles

### Clarity

Kumeo focuses on creating clear and intuitive experiences. The interface should effectively communicate the workflow between agents and events for both technical and non-technical users.

- **Visual simplicity** for complex concepts
- **Clear hierarchy** of information
- **Immediate visual feedback** for each action

### Consistency

Maintain visual consistency throughout the platform to create a familiar and predictable experience.

- **Consistent use** of colors, typography, and components
- **Uniform interaction patterns**
- **Coherent terminology** between UI and documentation

### Efficiency

Design for maximum user productivity, especially for repetitive tasks.

- **Keyboard shortcuts** for all main operations
- **Reduced steps** to complete common tasks
- **Customization** of the interface according to preferences

---

## Color Palette

The Kumeo palette is based on blue-green tones that convey trust, technology, and collaboration.

### Primary Colors

| Color | HEX Code | Usage |
|-------|----------|-------|
| Dark Teal | `#00796B` | Main brand color, headers, primary buttons |
| Green | `#28A745` | Secondary actions, success icons, favicon |
| Light Blue | `#4DBCE9` | Highlighted elements, active data |
| Very Light Blue | `#BFEFFF` | Backgrounds, hover states |

### Secondary Colors

| Color | HEX Code | Usage |
|-------|----------|-------|
| White | `#FFFFFF` | Backgrounds, text on dark colors |
| Light Greenish White | `#E9F7EF` | Alternate backgrounds, work areas |
| Medium Blue | `#6CB2EB` | Informational elements, links |
| Darker Teal | `#004D40` | Emphasis areas, footer |

### Semantic Usage

| Purpose | Color |
|---------|-------|
| Success | `#28A745` (Green) |
| Error | `#DC3545` (Not in palette - Red) |
| Warning | `#FFC107` (Not in palette - Yellow) |
| Information | `#4DBCE9` (Light Blue) |

---

## Typography

### Fonts

- **Primary**: Roboto - for most content
- **Secondary**: Open Sans - for headings and highlights

### Sizes

- **H1**: 2.5rem (40px)
- **H2**: 2rem (32px)
- **H3**: 1.5rem (24px)
- **Body**: 1rem (16px)
- **Small**: 0.875rem (14px)

### Usage

- Maintain high contrast between text and background (AA WCAG 2.0 minimum)
- Limit to no more than 2 font styles on a single screen
- Use bold for emphasis, avoid underlines except for links

---

## UI Components

### Buttons

**Primary Button**
- Background: `#00796B` (Dark Teal)
- Text: `#FFFFFF` (White)
- Hover states: 10% darker
- Border-radius: 0.5rem (8px)
- Padding: 1.5rem (24px) horizontal, 0.5rem (8px) vertical

**Secondary Button**
- Background: Transparent
- Border: `#00796B` (Dark Teal)
- Text: `#00796B` (Dark Teal)
- Hover states: Background with 10% opacity of primary color

**Tertiary/Ghost Button**
- No border or background
- Text: `#00796B` (Dark Teal)
- Hover states: Background with 5% opacity of primary color

### Cards

- Background: `#FFFFFF` (White)
- Border: None
- Border-radius: 0.5rem (8px)
- Shadow: `0 4px 8px rgba(0, 0, 0, 0.2)`
- Padding: 1.5rem (24px)

### Form Fields

- Height: 2.5rem (40px)
- Border-radius: 0.25rem (4px)
- Border: 1px solid `#E9ECEF` (Light Gray)
- Focus: `#4DBCE9` (Light Blue) border

### Navigation

- Navigation bar with `#00796B` (Dark Teal) background
- Text/icons in `#FFFFFF` (White)
- Active item: Underlined or background with 20% white

---

## Interaction Patterns

### Workflow Editor

**Visual Editor**
- Use drag-and-drop system to create flows
- Intuitive visual connections between agents
- Real-time preview of generated code
- Real-time visual validation

**Code Editor**
- Syntax highlighting with palette colors
- Intelligent autocomplete
- Descriptive error messages

### Agent Monitoring

- Color-differentiated status (active, inactive, error)
- Visual metrics for performance
- Filters to visualize different aspects of the system

### Contextual Navigation

- Breadcrumbs to reflect location in navigation
- Context menus for specific actions
- Accessible navigation history

---

## Responsiveness

Kumeo should work well on screens of different sizes, with special emphasis on development screens.

### Breakpoints

- **Small**: 576px
- **Medium**: 768px 
- **Large**: 992px
- **X-Large**: 1200px

### Mobile Considerations

- Prioritize read-only functions on mobile devices
- Change editor layouts to simplified views
- Allow visualization of running agent status

---

## Accessibility

Kumeo aims to be accessible to the largest possible number of users.

### Minimum Requirements

- WCAG AA text contrast (4.5:1 for normal text, 3:1 for large text)
- Complete keyboard navigation
- Screen reader compatibility
- Alternative text for all visual elements

### Best Practices

- Use ARIA when necessary to improve accessibility
- Test regularly with tools like Lighthouse or Axe
- Maintain visual and audio feedback for important actions

---

## Voice and Tone

### Communication Principles

- **Clear**: Avoid unnecessary jargon, explain complex concepts
- **Concise**: Direct and to-the-point messages
- **Technical but accessible**: Balance technical precision with comprehensibility
- **Empowering**: Guide the user, not just report errors

### Message Examples

**For success**
> "Workflow successfully implemented. 3 agents configured and running."

**For error**
> "Could not connect to NATS service. Check your connection and permissions."

**For status**
> "MLModel 'detector' processing data. 230/500 instances completed (46%)."

---

## Implementation

To implement these guidelines in Kumeo projects:

1. Import the base SCSS styles: `@import "branding/kumeo-brand.scss";`
2. Use the defined variables and mixins
3. Consult this guide for UX decisions not covered by the styles
4. Validate any new component against the established principles

By adhering to these guidelines, we ensure a consistent, accessible, and effective user experience throughout the Kumeo ecosystem.
