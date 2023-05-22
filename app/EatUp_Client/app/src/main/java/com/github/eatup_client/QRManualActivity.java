package com.github.eatup_client;

import android.content.Context;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.VibrationEffect;
import android.os.Vibrator;
import android.text.Editable;
import android.text.TextWatcher;
import android.util.Log;
import android.view.View;
import android.view.WindowManager;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;

public class QRManualActivity extends AppCompatActivity {

    private EditText[] edAuthWords = new EditText[3];
    private Button btnConfirmOTP;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qrmanual);
        this.getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        edAuthWords[0] = findViewById(R.id.edAuthWord1);
        edAuthWords[1] = findViewById(R.id.edAuthWord2);
        edAuthWords[2] = findViewById(R.id.edAuthWord3);
        btnConfirmOTP = findViewById(R.id.btnConfirmOTP);

        for (int i = 0; i < edAuthWords.length; i++) {
            edAuthWords[i].addTextChangedListener(new AuthWordTextWatcher(edAuthWords, i));
        }

        btnConfirmOTP.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                String authWords = "";
                for (EditText et : edAuthWords) {
                    authWords += et.getText().toString();
                }

                if (authWords.isEmpty()) {
                    Toast.makeText(getApplicationContext(), "Ingrese las palabras de autenticación", Toast.LENGTH_SHORT).show();
                    return;
                }

                if (isAuthWordsValid(authWords)) {
                    Toast.makeText(getApplicationContext(), "Palabras de autenticación válidas", Toast.LENGTH_SHORT).show();
                    Log.d("QRManualActivity", "Auth words: " + authWords);
                } else {
                    Toast.makeText(getApplicationContext(), "Palabras de autenticación inválidas", Toast.LENGTH_SHORT).show();
                    Log.d("QRManualActivity", "ERROR auth words: " + authWords);
                    sendVibration(500);
                }
            }
        });
    }

    private boolean isAuthWordsValid(String s) {
        // TODO: Implementar la lógica de validación de las palabras de autenticación
        // Aquí puedes verificar si las palabras ingresadas son válidas según tus requisitos
        return s.length() > 0;
    }

    private void sendVibration(int i) {
        Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            v.vibrate(VibrationEffect.createOneShot(i, VibrationEffect.DEFAULT_AMPLITUDE));
        } else {
            v.vibrate(i);
        }
    }

    private class AuthWordTextWatcher implements TextWatcher {

        private EditText[] editTexts;
        private int index;
        private Handler handler;
        private boolean isTyping;

        public AuthWordTextWatcher(EditText[] editTexts, int index) {
            this.editTexts = editTexts;
            this.index = index;
            this.handler = new Handler();
            this.isTyping = false;
        }

        @Override
        public void beforeTextChanged(CharSequence s, int start, int count, int after) {
        }

        @Override
        public void onTextChanged(CharSequence s, int start, int before, int count) {
            if (count > 0 && s.charAt(start) == ' ') {
                // If space is entered, remove it and move to the next EditText
                String updatedText = s.toString().replaceAll(" ", "");
                editTexts[index].setText(updatedText);

                if (index < editTexts.length - 1) {
                    editTexts[index + 1].requestFocus();
                }
            }
        }

        @Override
        public void afterTextChanged(Editable s) {
            // Change focus to the next EditText when the user finishes typing
            if (s.length() > 0 && !isTyping) {
                isTyping = true;
                handler.postDelayed(() -> {
                    isTyping = false;
                    if (index < editTexts.length - 1) {
                        editTexts[index + 1].requestFocus();
                    }
                }, 1000);
            }
        }
    }
}