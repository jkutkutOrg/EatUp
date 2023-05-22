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
import android.view.WindowManager;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.lifecycle.LiveData;

import com.github.eatup_client.api.ProductApiService;

public class QRManualActivity extends AppCompatActivity {

    private EditText[] edAuthWords = new EditText[3];
    private Button btnConfirmOTP;
    private ProductApiService productApiService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qrmanual);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        productApiService = new ProductApiService(this);

        // Find all EditText views
        for (int i = 0; i < edAuthWords.length; i++) {
            edAuthWords[i] = findViewById(R.id.edAuthWord1 + i);
            edAuthWords[i].addTextChangedListener(new AuthWordTextWatcher(edAuthWords, i));
        }

        btnConfirmOTP = findViewById(R.id.btnConfirmOTP);
        btnConfirmOTP.setOnClickListener(v -> {
            StringBuilder authWords = new StringBuilder();
            int lastIndex = edAuthWords.length - 1;
            for (int i = 0; i < edAuthWords.length; i++) {
                authWords.append(edAuthWords[i].getText().toString().trim());
                if (i != lastIndex) {
                    authWords.append(" ");
                }
            }

            if (authWords.toString().isEmpty()) {
                Toast.makeText(getApplicationContext(), "Ingrese las palabras de autenticación", Toast.LENGTH_SHORT).show();
                return;
            }

            isAuthWordsValid(authWords.toString());
        });
    }

    private void isAuthWordsValid(String authWords) {
        // Method: getValidSession
        LiveData<Boolean> validSessionLiveData = productApiService.getValidSession(authWords);

        validSessionLiveData.observe(this, isValidSession -> {
            if (isValidSession) {
                // The session is valid
                Toast.makeText(getApplicationContext(), "Palabras de autenticación válidas", Toast.LENGTH_SHORT).show();
                Log.d("QRManualActivity", "Auth words: " + authWords);
            } else {
                // The session is invalid
                Toast.makeText(getApplicationContext(), "Palabras de autenticación inválidas", Toast.LENGTH_SHORT).show();
                // Vibrate the device to indicate an error
                Vibrator vibrator = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    vibrator.vibrate(VibrationEffect.createOneShot(500, VibrationEffect.DEFAULT_AMPLITUDE));
                } else {
                    vibrator.vibrate(500);
                }
                Log.d("QRManualActivity", "ERROR auth words: " + authWords);
            }
        });
    }

    private class AuthWordTextWatcher implements TextWatcher {

        private EditText[] mEditTexts;
        private int mCurrentIndex;
        private Handler mHandler;
        private boolean mIsTyping = false;

        public AuthWordTextWatcher(EditText[] editTexts, int currentIndex) {
            this.mEditTexts = editTexts;
            this.mCurrentIndex = currentIndex;
            this.mHandler = new Handler();
        }

        @Override
        public void beforeTextChanged(CharSequence s, int start, int count, int after) {
            mIsTyping = count > 0;
        }

        @Override
        public void onTextChanged(CharSequence s, int start, int before, int count) {
            if (count > 0 && s.charAt(start) == ' ') {
                // If space is entered, remove it and move to the next EditText
                mEditTexts[mCurrentIndex].setText(s.toString().replaceAll(" ", "").trim());
                if (mCurrentIndex < mEditTexts.length - 1) {
                    mEditTexts[++mCurrentIndex].requestFocus();
                }
            }
        }

        @Override
        public void afterTextChanged(Editable s) {
            if (s.length() == 0) {
                // Move focus to previous EditText if user deletes text
                if (mCurrentIndex > 0) {
                    mEditTexts[--mCurrentIndex].requestFocus();
                }
            } else if (mIsTyping) {
                mHandler.postDelayed(() -> {
                    if (mCurrentIndex < mEditTexts.length - 1) {
                        mEditTexts[++mCurrentIndex].requestFocus();
                    }
                }, 1000);
            }
        }
    }
}