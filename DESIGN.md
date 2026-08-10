# RepoRadar Design System

## 1. Atmosphere & Identity

RepoRadar is a quiet developer command center: dense enough for repository work, but calm enough that the current context is always obvious. Its signature is a dark tonal workspace with a restrained indigo signal for active navigation, ready states, and primary actions.

## 2. Color

### Palette

| Role | Token | Value | Usage |
|------|-------|-------|-------|
| Surface/base | `--color-surface-base` | `#090b0f` | App background |
| Surface/raised | `--color-surface-raised` | `#101319` | Cards and views |
| Surface/elevated | `--color-surface-elevated` | `#151a24` | Selected rows and active panels |
| Surface/control | `--color-surface-control` | `#171c27` | Buttons and inputs |
| Surface/titlebar | `--color-surface-titlebar` | `#0c0e13` | Native-style app chrome |
| Surface/input | `--color-surface-input` | `#11141b` | Search and form controls |
| Surface/progress | `--color-surface-progress` | `#11182d` | Scan progress feedback |
| Text/primary | `--color-text-primary` | `#f3f4f6` | Headings and primary labels |
| Text/secondary | `--color-text-secondary` | `#9ca6b8` | Supporting copy |
| Text/muted | `--color-text-muted` | `#737987` | Metadata and disabled copy |
| Text/soft | `--color-text-soft` | `#898f9c` | Secondary descriptive copy |
| Text/link | `--color-text-link` | `#aeb6ff` | Interactive and progress copy |
| Text/on-control | `--color-text-on-control` | `#dce0ea` | Text on raised controls |
| Border/default | `--color-border-default` | `#20242d` | View and card boundaries |
| Border/strong | `--color-border-strong` | `#343c52` | Hover and selected boundaries |
| Border/control | `--color-border-control` | `#292e3a` | Inputs and form controls |
| Border/progress | `--color-border-progress` | `#2e385d` | Scan progress feedback |
| Accent/primary | `--color-accent-primary` | `#6876ff` | Active tabs, focus, primary action |
| Accent/soft | `--color-accent-soft` | `#222745` | Badges and selected controls |
| Accent/track | `--color-accent-track` | `rgba(104, 118, 255, 0.25)` | Spinner tracks and quiet motion |
| Status/success | `--color-status-success` | `#67d3a4` | Ready and completed states |
| Status/warning | `--color-status-warning` | `#f0c36e` | Running state |
| Status/error | `--color-status-error` | `#ef7789` | Errors and cancellation |

Accent is interactive only. Depth uses tonal shifts and restrained borders; shadows remain limited to brand emphasis and focus glow.

## 3. Typography

| Level | Size | Weight | Usage |
|-------|------|--------|-------|
| Display | `clamp(30px, 4vw, 46px)` | 700 | Overview title |
| H2 | 22px | 600 | View and hero headings |
| H3 | 18px | 600 | Cards and project titles |
| Body | 14–15px | 400 | Descriptions and readable content |
| Caption | 10–12px | 500–700 | Labels and metadata |

Primary font is the existing system UI stack (`Inter`, `SF Pro Display`, `Segoe UI`, `Microsoft YaHei`, sans-serif). Repository paths, branches, hashes, and status values use `Cascadia Code`, `Consolas`, monospace.

## 4. Spacing & Layout

Spacing uses the shared `--space-*` scale (4–32px) plus semantic shell/component tokens for exact chrome dimensions. Typography uses named `--font-size-*` tokens for display, title, body, metadata, detail, and operation text; radii use `--radius-*`. The desktop shell is a fixed-width sidebar plus a flexible content region. The main region owns one view viewport; the project view owns two named local scroll regions: project list and project detail. At mobile widths the sidebar becomes a horizontal tab rail and the project panes stack into one readable column.

## 5. Components

### Primary navigation tabs

- **Structure**: labelled buttons inside a `nav`.
- **Variants**: default, active, disabled, compact mobile.
- **Spacing**: 8px gap, 42px minimum hit area, 12px inline padding.
- **States**: hover, active, focus-visible, disabled.
- **Accessibility**: `aria-current="page"`, keyboard reachable, disabled GitHub when no project is selected.
- **Motion**: active indicator and surface changes use 200ms opacity/background transitions only.
- **Layout**: fixed sidebar on desktop; horizontal overflow-safe tab rail on mobile.

### View viewport

- **Structure**: one active `section` below the persistent header and feedback strip.
- **Variants**: overview, projects, GitHub, operations.
- **States**: loading, ready, empty, error.
- **Accessibility**: active view has a descriptive heading; loading regions expose `aria-busy`.
- **Motion**: view entry uses a short opacity transition; reduced motion disables it.
- **Layout**: the viewport is the only primary scroll owner.

### Project split workspace

- **Structure**: project catalog pane plus selected project detail pane.
- **Variants**: no project, project loading, project ready, filtered empty.
- **States**: selected, busy, loading, empty, error.
- **Accessibility**: project rows are buttons with visible focus and selected styling.
- **Motion**: selected row and detail loading transition use opacity/background only.
- **Layout**: two local scroll panes on desktop; stacked on mobile.

### Loading feedback

- **Structure**: block skeletons for initial data, inline button progress for actions, determinate progress for directory scans.
- **States**: loading, running, success, failed.
- **Accessibility**: `aria-busy`, `aria-live`, and text labels remain present when motion is reduced.
- **Motion**: calm opacity pulse; no layout animation.

## 6. Motion & Interaction

Micro interactions use 120–160ms ease-out. View and tab transitions use 200ms ease-in-out. Progress indicators may animate transform, but every non-essential animation is disabled under `prefers-reduced-motion: reduce`; the corresponding state text remains visible. No interaction changes layout dimensions during loading.

## 7. Depth & Surface

Use the mixed strategy already present: tonal-shift surfaces separated by subtle borders. Selected navigation and project rows use the elevated surface plus the accent border; no new decorative gradients are introduced.

## 8. Accessibility Constraints & Accepted Debt

Target WCAG 2.2 AA: body contrast at least 4.5:1, visible focus for every control, keyboard reachability, no horizontal overflow at 375px, and reduced motion support.

| Item | Location | Why accepted | Exit |
|------|----------|--------------|------|
| Existing raw color literals | `src/styles.css` legacy selectors | Preserve the current visual baseline during the structural refactor | Consolidate during a dedicated theme pass |
| Text glyph marks in legacy chrome | `src/components/AppSidebar.vue` and related views | No icon dependency is currently installed | Replace with a shared SVG icon set in a separate asset pass |
