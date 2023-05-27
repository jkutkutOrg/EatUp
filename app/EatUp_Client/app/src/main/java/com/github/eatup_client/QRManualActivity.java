package com.github.eatup_client;

import android.content.Context;
import android.content.Intent;
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
import com.github.eatup_client.model.SessionId;

public class QRManualActivity extends AppCompatActivity {

    private static final int AUTH_WORDS_COUNT = 3;

    private EditText[] edAuthWords = new EditText[AUTH_WORDS_COUNT];
    private Button btnConfirmOTP;
    private Button bntReturnToScan;
    private ProductApiService productApiService;
    private static final String TAG = "QRManualActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qrmanual);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        productApiService = new ProductApiService(this);

        for (int i = 0; i < AUTH_WORDS_COUNT; i++) {
            int editTextId = getResources().getIdentifier("edAuthWord" + (i + 1), "id", getPackageName());
            edAuthWords[i] = findViewById(editTextId);
            edAuthWords[i].addTextChangedListener(new AuthWordTextWatcher(i));
        }

        btnConfirmOTP = findViewById(R.id.btnConfirmOTP);
        btnConfirmOTP.setOnClickListener(v -> {
            StringBuilder authWords = new StringBuilder();
            int lastIndex = AUTH_WORDS_COUNT - 1;
            for (int i = 0; i < AUTH_WORDS_COUNT; i++) {
                authWords.append(edAuthWords[i].getText().toString().trim());
                if (i != lastIndex) {
                    authWords.append(" ");
                }
            }

            if (authWords.toString().isEmpty()) {
                showToast("Please enter the authentication words");
                return;
            }

            validateAuthWords(authWords.toString());
        });

        bntReturnToScan = findViewById(R.id.bntReturnToScan);
        bntReturnToScan.setOnClickListener(v -> {
            Intent intent = new Intent(QRManualActivity.this, QRActivity.class);
            startActivity(intent);
        });

    }

    private void validateAuthWords(String authWords) {
        LiveData<SessionId> validSessionLiveData = productApiService.getValidSession(authWords);

        validSessionLiveData.observe(this, sessionId -> {
            if (sessionId != null) {
                navigateToMenuActivity();
                Log.d(TAG, "SUCCESS auth words: " + authWords);
            } else {
                showToast("Invalid authentication words");
                vibrate();
                Log.d(TAG, "FAILED auth words: " + authWords);
            }
        });
    }

    private void navigateToMenuActivity() {
        Intent intent = new Intent(QRManualActivity.this, MenuActivity.class);
        startActivity(intent);
    }

    private void showToast(String message) {
        Toast.makeText(getApplicationContext(), message, Toast.LENGTH_SHORT).show();
    }

    private void vibrate() {
        Vibrator vibrator = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            vibrator.vibrate(VibrationEffect.createOneShot(500, VibrationEffect.DEFAULT_AMPLITUDE));
        } else {
            vibrator.vibrate(500);
        }
    }

    private class AuthWordTextWatcher implements TextWatcher {

        private int mCurrentIndex;
        private Handler mHandler;
        private boolean mIsTyping = false;

        public AuthWordTextWatcher(int currentIndex) {
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
                edAuthWords[mCurrentIndex].setText(s.toString().replaceAll(" ", "").trim());
                if (mCurrentIndex < AUTH_WORDS_COUNT - 1) {
                    edAuthWords[++mCurrentIndex].requestFocus();
                }
            }
        }

        @Override
        public void afterTextChanged(Editable s) {
            if (s.length() == 0) {
                if (mCurrentIndex > 0) {
                    edAuthWords[--mCurrentIndex].requestFocus();
                }
            } else if (mIsTyping) {
                mHandler.postDelayed(() -> {
                    if (mCurrentIndex < AUTH_WORDS_COUNT - 1) {
                        edAuthWords[++mCurrentIndex].requestFocus();
                    }
                }, 1000);
            }
        }
    }
}