package com.github.eatup_client;

import android.content.Context;
import android.os.Build;
import android.os.Bundle;
import android.os.VibrationEffect;
import android.os.Vibrator;
import android.text.Editable;
import android.text.TextWatcher;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;

public class QRManualActivity extends AppCompatActivity {

    private EditText[] edOTPCode = new EditText[6];
    private Button btnConfirmOTP;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qrmanual);

        edOTPCode[0] = findViewById(R.id.edOTPCode1);
        edOTPCode[1] = findViewById(R.id.edOTPCode2);
        edOTPCode[2] = findViewById(R.id.edOTPCode3);
        edOTPCode[3] = findViewById(R.id.edOTPCode4);
        edOTPCode[4] = findViewById(R.id.edOTPCode5);
        edOTPCode[5] = findViewById(R.id.edOTPCode6);
        btnConfirmOTP = findViewById(R.id.btnConfirmOTP);

        for (int i = 0; i < edOTPCode.length; i++) {
            edOTPCode[i].addTextChangedListener(new OTPTextWatcher(edOTPCode, i));
        }

        btnConfirmOTP.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                String otpCode = "";
                for (EditText et : edOTPCode) {
                    otpCode += et.getText().toString();
                }

                if (otpCode.length() != 6) {
                    Toast.makeText(getApplicationContext(), "Introduzca un código válido", Toast.LENGTH_SHORT).show();
                    Log.d("QRManualActivity", "ERROR OTP code length: " + otpCode.length());
                    return;
                }

                if (isOTPValid(otpCode)) {
                    Toast.makeText(getApplicationContext(), "Código OTP válido", Toast.LENGTH_SHORT).show();
                    Log.d("QRManualActivity", "OTP code: " + otpCode);
                } else {
                    Toast.makeText(getApplicationContext(), "Código OTP inválido", Toast.LENGTH_SHORT).show();
                    Log.d("QRManualActivity", "ERROR OTP code: " + otpCode);
                    sendVibration(500);
                }
            }
        });

        edOTPCode[edOTPCode.length - 1].addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {
            }

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {
            }

            @Override
            public void afterTextChanged(Editable s) {
                if (s.length() == 1 && isAllFieldsCompleted()) {
                    String otpCode = getOTPCode();

                    if (otpCode.length() != 6) {
                        showToast("Introduzca un código válido");
                        Log.d("QRManualActivity", "ERROR OTP code length: " + otpCode.length());
                        return;
                    }

                    if (isOTPValid(otpCode)) {
                        showToast("Código OTP válido");
                        Log.d("QRManualActivity", "OTP code: " + otpCode);
                    } else {
                        showToast("Código OTP inválido");
                        Log.d("QRManualActivity", "ERROR OTP code: " + otpCode);
                        sendVibration(500);
                    }
                }
            }
        });

    }

    private boolean isOTPValid(String s) {
        // TODO: Pending to implement the OTP validation
        return s.matches("[0-9]+") && s.length() == 6;
    }

    private void sendVibration(int i) {
        Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            v.vibrate(VibrationEffect.createOneShot(i, VibrationEffect.DEFAULT_AMPLITUDE));
        } else {
            v.vibrate(i);
        }
    }

    private void showToast(String s) {
        Toast.makeText(getApplicationContext(), s, Toast.LENGTH_SHORT).show();
    }

    private String getOTPCode() {
        String otpCode = "";
        for (EditText et : edOTPCode) {
            otpCode += et.getText().toString();
        }
        return otpCode;
    }

    private boolean isAllFieldsCompleted() {
        for (EditText et : edOTPCode) {
            if (et.getText().toString().isEmpty()) {
                return false;
            }
        }
        return true;
    }

    private class OTPTextWatcher implements TextWatcher {

        private EditText[] editTexts;
        private int index;

        public OTPTextWatcher(EditText[] editTexts, int index) {
            this.editTexts = editTexts;
            this.index = index;
        }

        @Override
        public void beforeTextChanged(CharSequence s, int start, int count, int after) {
        }

        @Override
        public void onTextChanged(CharSequence s, int start, int before, int count) {
        }

        @Override
        public void afterTextChanged(Editable s) {
            if (s.length() == 1 && index < editTexts.length - 1) {
                editTexts[index + 1].requestFocus();
            } else if (s.length() == 0 && index > 0) {
                editTexts[index - 1].requestFocus();
            }
        }
    }
}