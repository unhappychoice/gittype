# Themes & Customization

GitType provides extensive theming capabilities to personalize your typing experience with beautiful color schemes and custom styling options.

## Built-in Themes

GitType comes with 15 carefully crafted themes to match your coding style and environment:

| Theme | Description |
|-------|-------------|
| `default` | Balanced palette for comfortable readability |
| `original` | Classic GitType color scheme |
| `ascii` | Monochrome terminal aesthetic |
| `aurora` | Northern lights inspired |
| `blood_oath` | Dark red vampire theme |
| `cyber_void` | Futuristic neon cyberpunk |
| `eclipse` | Deep space darkness |
| `glacier` | Cool blue ice tones |
| `inferno` | Hot fire and lava colors |
| `neon_abyss` | Electric neon in darkness |
| `oblivion` | Mysterious dark void |
| `runic` | Ancient mystical symbols |
| `spectral` | Ghostly ethereal colors |
| `starforge` | Cosmic star creation |
| `venom` | Toxic green poison theme |

## Changing Themes

### From Settings Screen
1. Press **Esc** from any screen to return to main menu
2. Navigate to **Settings** → **Theme**
3. Use **Up/Down** arrows to browse themes
4. Changes are applied instantly for preview
5. Press **Enter** to confirm selection

### Color Mode Toggle
Switch between Dark and Light modes:
- Each theme supports both color modes
- Access via **Settings** → **Color Mode**
- Dark mode is optimized for low-light environments
- Light mode provides better contrast in bright conditions

## Creating Custom Themes

### Theme File Structure
Custom theme files use simplified JSON format without metadata:

```json
{
  "dark": {
    "border": {"r": 102, "g": 153, "b": 204},
    "title": {"r": 235, "g": 235, "b": 235},
    "text": {"r": 220, "g": 220, "b": 220},
    "background": {"r": 15, "g": 15, "b": 15},
    // ... more color definitions
  },
  "light": {
    "border": {"r": 120, "g": 160, "b": 220},
    // ... light mode colors
  }
}
```

Note: The custom theme automatically appears as "Custom" in the theme selection menu.

### Color Properties
Each theme defines colors for both `dark` and `light` modes:

#### Interface Elements
- `border` - Window borders and dividers
- `title` - Main titles and headings
- `text` - Primary text content
- `text_secondary` - Secondary/disabled text
- `background` - Main background
- `background_secondary` - Secondary panels

#### Status Colors
- `status_success` - Success messages (typically green)
- `status_info` - Information messages (typically blue)
- `status_warning` - Warning messages (typically yellow)
- `status_error` - Error messages (typically red)

#### Key Indicators
- `key_back` - Back/cancel actions (usually red)
- `key_action` - Confirm/action keys (usually green)
- `key_navigation` - Navigation keys (usually blue)

#### Typing Interface
- `typing_untyped_text` - Text not yet typed
- `typing_typed_text` - Successfully typed text
- `typing_cursor_fg` - Cursor text color
- `typing_cursor_bg` - Cursor background
- `typing_mistake_bg` - Error highlight background

#### Metrics Display
- `metrics_score` - Score display
- `metrics_cpm_wpm` - Speed metrics (CPM/WPM)
- `metrics_accuracy` - Accuracy percentage
- `metrics_duration` - Timer display
- `metrics_stage_info` - Stage information

### Installing Custom Theme

1. **Create theme file**: Save as `~/.gittype/custom-theme.json`
2. **Restart GitType**: The theme will be available as "Custom" in Settings → Theme menu
3. **File location**: Custom theme must be saved as `~/.gittype/custom-theme.json`

### Complete Theme Example

Here's a complete example creating a custom theme:

```json
{
  "dark": {
    "border": {"r": 0, "g": 255, "b": 0},
    "title": {"r": 0, "g": 255, "b": 0},
    "text": {"r": 0, "g": 200, "b": 0},
    "text_secondary": {"r": 0, "g": 100, "b": 0},
    "background": {"r": 0, "g": 0, "b": 0},
    "background_secondary": {"r": 10, "g": 10, "b": 10},
    "status_success": {"r": 0, "g": 255, "b": 0},
    "status_info": {"r": 0, "g": 200, "b": 0},
    "status_warning": {"r": 200, "g": 255, "b": 0},
    "status_error": {"r": 255, "g": 100, "b": 100},
    "key_back": {"r": 255, "g": 100, "b": 100},
    "key_action": {"r": 0, "g": 255, "b": 0},
    "key_navigation": {"r": 0, "g": 200, "b": 0},
    "metrics_score": {"r": 0, "g": 255, "b": 0},
    "metrics_cpm_wpm": {"r": 0, "g": 200, "b": 0},
    "metrics_accuracy": {"r": 0, "g": 255, "b": 0},
    "metrics_duration": {"r": 0, "g": 200, "b": 0},
    "metrics_stage_info": {"r": 0, "g": 150, "b": 0},
    "typing_untyped_text": {"r": 0, "g": 150, "b": 0},
    "typing_typed_text": {"r": 0, "g": 255, "b": 0},
    "typing_cursor_fg": {"r": 0, "g": 0, "b": 0},
    "typing_cursor_bg": {"r": 0, "g": 255, "b": 0},
    "typing_mistake_bg": {"r": 100, "g": 0, "b": 0}
  },
  "light": {
    "border": {"r": 0, "g": 150, "b": 0},
    "title": {"r": 0, "g": 100, "b": 0},
    "text": {"r": 0, "g": 80, "b": 0},
    "text_secondary": {"r": 100, "g": 120, "b": 100},
    "background": {"r": 240, "g": 255, "b": 240},
    "background_secondary": {"r": 220, "g": 240, "b": 220},
    "status_success": {"r": 0, "g": 120, "b": 0},
    "status_info": {"r": 0, "g": 100, "b": 0},
    "status_warning": {"r": 150, "g": 150, "b": 0},
    "status_error": {"r": 150, "g": 0, "b": 0},
    "key_back": {"r": 150, "g": 0, "b": 0},
    "key_action": {"r": 0, "g": 120, "b": 0},
    "key_navigation": {"r": 0, "g": 100, "b": 0},
    "metrics_score": {"r": 0, "g": 120, "b": 0},
    "metrics_cpm_wpm": {"r": 0, "g": 100, "b": 0},
    "metrics_accuracy": {"r": 0, "g": 120, "b": 0},
    "metrics_duration": {"r": 0, "g": 100, "b": 0},
    "metrics_stage_info": {"r": 0, "g": 80, "b": 0},
    "typing_untyped_text": {"r": 100, "g": 120, "b": 100},
    "typing_typed_text": {"r": 0, "g": 100, "b": 0},
    "typing_cursor_fg": {"r": 255, "g": 255, "b": 255},
    "typing_cursor_bg": {"r": 0, "g": 120, "b": 0},
    "typing_mistake_bg": {"r": 255, "g": 200, "b": 200}
  }
}
```

Save this as `~/.gittype/custom-theme.json` and restart GitType to use it.

## Configuration Files

### Theme Settings
Your current theme preference is stored in `~/.gittype/config.json`:

```json
{
  "theme": {
    "current_theme_id": "glacier",
    "current_color_mode": "Dark"
  }
}
```

### Custom Theme Storage
- **Location**: `~/.gittype/custom-theme.json`
- **Format**: JSON format without theme metadata (id, name, description)
- **Auto-detection**: GitType automatically loads the custom theme if the file exists

## Color Format

Colors use RGB format with values from 0-255:
```json
{"r": 255, "g": 128, "b": 64}
```

## Tips for Theme Creation

1. **Test both modes**: Always define both dark and light variants
2. **Maintain contrast**: Ensure sufficient contrast for readability
3. **Consistent palette**: Use a cohesive color scheme
4. **Accessibility**: Consider colorblind-friendly palettes
5. **Preview instantly**: Changes in Settings show immediately for testing

## Troubleshooting

**Theme not appearing**:
- Check JSON syntax with a validator
- Ensure all required color properties are defined
- Restart GitType after adding new themes

**Colors not displaying correctly**:
- Verify RGB values are within 0-255 range
- Check that both "dark" and "light" sections are complete

**Theme reverts to default**:
- Check that `~/.gittype/config.json` has proper write permissions
- Verify the custom theme file is named exactly `custom-theme.json`