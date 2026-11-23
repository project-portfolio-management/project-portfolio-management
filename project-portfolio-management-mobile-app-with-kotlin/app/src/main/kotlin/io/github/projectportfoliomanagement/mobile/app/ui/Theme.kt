// MA-4: Lily's light or dark theme, following the system. Lily draws its own components; Material 3, coloured from the same
// tokens, draws the native controls Lily does not ship. Lily's text styles take Material's type scale, in sp, so the
// person's font size applies to both.
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import io.github.lilydesignsystem.compose.LilyTextStyles
import io.github.lilydesignsystem.compose.LocalLilyTextStyles
import io.github.lilydesignsystem.compose.themes.LilyTheme
import io.github.lilydesignsystem.compose.themes.LilyTokens

@Composable
fun PpmTheme(dark: Boolean = isSystemInDarkTheme(), content: @Composable () -> Unit) {
    val tokens = if (dark) LilyTokens.dark else LilyTokens.light
    val base = if (dark) darkColorScheme() else lightColorScheme()
    val colors = base.copy(
        primary = tokens.primary,
        onPrimary = tokens.primaryContent,
        secondary = tokens.secondary,
        onSecondary = tokens.secondaryContent,
        background = tokens.base100,
        onBackground = tokens.baseContent,
        surface = tokens.base100,
        onSurface = tokens.baseContent,
        surfaceContainer = tokens.base200,
        surfaceContainerHigh = tokens.base200,
        surfaceContainerHighest = tokens.base200,
        onSurfaceVariant = tokens.baseContent,
        error = tokens.error,
        onError = tokens.errorContent,
    )
    LilyTheme(tokens) {
        MaterialTheme(colorScheme = colors) {
            val type = MaterialTheme.typography
            val text = TextStyle(color = tokens.baseContent)
            CompositionLocalProvider(
                LocalLilyTextStyles provides LilyTextStyles(
                    body = type.bodyLarge.merge(text),
                    strong = type.bodyLarge.merge(text).copy(fontWeight = FontWeight.Bold),
                    heading = type.titleMedium.merge(text).copy(fontWeight = FontWeight.Bold),
                    small = type.bodyMedium.merge(text),
                ),
                content = content,
            )
        }
    }
}
