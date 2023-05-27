package com.github.eatup_client;

import android.Manifest;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.os.Vibrator;
import android.util.Log;
import android.util.SparseArray;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.WindowManager;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;
import androidx.lifecycle.LiveData;

import com.github.eatup_client.api.ProductApiService;
import com.google.android.gms.vision.CameraSource;
import com.google.android.gms.vision.Detector;
import com.google.android.gms.vision.barcode.Barcode;
import com.google.android.gms.vision.barcode.BarcodeDetector;

import java.io.IOException;

public class QRActivity extends AppCompatActivity {

    private SurfaceView sfvQR;
    private TextView tvQR;
    private Button btnProblemScanner;
    private BarcodeDetector barcodeDetector;
    private CameraSource cameraSource;
    private ProductApiService productApiService;
    private static final int REQUEST_CAMERA_PERMISSION = 201;
    private boolean cameraPermissionGranted = false;
    private static final String TAG = "QRActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qr);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN, WindowManager.LayoutParams.FLAG_FULLSCREEN);

        tvQR = findViewById(R.id.tvWelcomeTitle);
        sfvQR = findViewById(R.id.sfvQR);
        btnProblemScanner = findViewById(R.id.btnProblemScanner);

        btnProblemScanner.setOnClickListener(v -> {
            Intent intent = new Intent(QRActivity.this, QRManualActivity.class);
            if (intent.resolveActivity(getPackageManager()) != null) {
                startActivity(intent);
            }
        });

        productApiService = new ProductApiService(this);
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (cameraPermissionGranted) {
            initCameraAndDetector();
        } else {
            requestCameraPermission();
        }
    }

    private void requestCameraPermission() {
        if (ActivityCompat.checkSelfPermission(this, Manifest.permission.CAMERA) == PackageManager.PERMISSION_GRANTED) {
            cameraPermissionGranted = true;
            initCameraAndDetector();
        } else {
            ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.CAMERA}, REQUEST_CAMERA_PERMISSION);
        }
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        if (requestCode == REQUEST_CAMERA_PERMISSION && grantResults.length > 0 && grantResults[0] == PackageManager.PERMISSION_GRANTED) {
            cameraPermissionGranted = true;
            Intent intent = new Intent(QRActivity.this, MainActivity.class);
            startActivity(intent);
        } else {
            Toast.makeText(this, "Camera permission is required to scan QR codes", Toast.LENGTH_SHORT).show();
            Intent intent = new Intent(QRActivity.this, QRManualActivity.class);
            startActivity(intent);
        }
    }

    private void initCameraAndDetector() {
        Log.d(TAG, "Initializing camera and detector");

        barcodeDetector = new BarcodeDetector.Builder(this)
                .setBarcodeFormats(Barcode.ALL_FORMATS)
                .build();

        cameraSource = new CameraSource.Builder(this, barcodeDetector)
                .setRequestedPreviewSize(1920, 1080)
                .setAutoFocusEnabled(true)
                .build();

        sfvQR.getHolder().addCallback(new SurfaceHolder.Callback() {
            @Override
            public void surfaceCreated(SurfaceHolder holder) {
                startCamera();
            }

            @Override
            public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
            }

            @Override
            public void surfaceDestroyed(SurfaceHolder holder) {
                stopCamera();
            }
        });

        barcodeDetector.setProcessor(new Detector.Processor<Barcode>() {
            @Override
            public void release() {
                Log.d(TAG, "Barcode detector released");
            }

            @Override
            public void receiveDetections(@NonNull Detector.Detections<Barcode> detections) {
                final SparseArray<Barcode> barcodes = detections.getDetectedItems();
                if (barcodes.size() == 0) {
                    return;
                }

                tvQR.post(() -> {
                    String qrCode = barcodes.valueAt(0).displayValue;
                    Log.d(TAG, "QR code detected: " + qrCode);
                    LiveData<Boolean> isValidLiveData = productApiService.isQRValid(qrCode);
                    Log.d(TAG, "QR code is valid: " + isValidLiveData.getValue());
                    isValidLiveData.observe(QRActivity.this, isValid -> {
                        if (isValid != null && isValid) {
                            Intent intent = new Intent(QRActivity.this, MenuActivity.class);
                            startActivity(intent);
                        } else {
                            Vibrator vibrator = (Vibrator) getSystemService(VIBRATOR_SERVICE);
                            if (vibrator != null) {
                                vibrator.vibrate(100);
                            }
                            tvQR.setText("Invalid QR code\nPlease try again");
                            Log.d(TAG, "Invalid QR code, changing text and vibrating");
                        }
                    });
                });
            }
        });
    }

    private void startCamera() {
        try {
            if (ActivityCompat.checkSelfPermission(this, Manifest.permission.CAMERA) != PackageManager.PERMISSION_GRANTED) {
                Log.e(TAG, "Camera permission not granted");
                return;
            }
            cameraSource.start(sfvQR.getHolder());
            Log.d(TAG, "Camera started");
        } catch (IOException e) {
            e.printStackTrace();
            Log.e(TAG, "Failed to start camera: " + e.getMessage());
        }
    }

    private void stopCamera() {
        if (cameraSource != null) {
            cameraSource.stop();
            Log.d(TAG, "Camera stopped");
        }
    }

    @Override
    protected void onPause() {
        super.onPause();
        stopCamera();
    }
}
