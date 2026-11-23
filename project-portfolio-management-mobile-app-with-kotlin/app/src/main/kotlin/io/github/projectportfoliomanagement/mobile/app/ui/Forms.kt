// A text field inside Lily's Field: Field gives the label, description, error and the spoken name; Material 3 draws the
// control (MA-4).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.OutlinedTextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import io.github.lilydesignsystem.compose.Field
import io.github.projectportfoliomanagement.mobile.app.R
import androidx.compose.ui.res.stringResource

@Composable
fun TextFieldRow(
    label: String,
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    description: String? = null,
    error: String? = null,
    required: Boolean = false,
    focusRequester: FocusRequester? = null,
    keyboardOptions: KeyboardOptions = KeyboardOptions.Default,
    keyboardActions: KeyboardActions = KeyboardActions.Default,
) {
    Field(
        label = label,
        description = description,
        error = error,
        required = required,
        requiredLabel = if (required) stringResource(R.string.required) else null,
        modifier = modifier.fillMaxWidth(),
    ) { fieldModifier ->
        OutlinedTextField(
            value = value,
            onValueChange = onValueChange,
            isError = error != null,
            keyboardOptions = keyboardOptions,
            keyboardActions = keyboardActions,
            modifier = fieldModifier
                .fillMaxWidth()
                .let { if (focusRequester != null) it.focusRequester(focusRequester) else it },
        )
    }
}
