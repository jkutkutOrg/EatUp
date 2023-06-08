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

import com.github.eatup_client.api.ProductApiService;
import com.google.android.gms.vision.CameraSource;
import com.google.android.gms.vision.Detector;
import com.google.android.gms.vision.barcode.Barcode;
import com.google.android.gms.vision.barcode.BarcodeDetector;

import java.io.IOException;

/**
 * Activity for scanning QR codes using the device camera.
 * Validates the scanned QR code and navigates to the menu screen if valid.
 */
public class QRActivity extends AppCompatActivity {

    private static final String TAG = "QRActivity";
    private static final int REQUEST_CAMERA_PERMISSION = 201;

    private SurfaceView sfvQR;
    private TextView tvQR;
    private Button btnProblemScanner;

    private BarcodeDetector barcodeDetector;
    private CameraSource cameraSource;

    private ProductApiService productApiService;
    private Vibrator vibrator;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qr);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN, WindowManager.LayoutParams.FLAG_FULLSCREEN);

        // Initialize UI components
        tvQR = findViewById(R.id.tvWelcomeTitle);
        sfvQR = findViewById(R.id.sfvQR);
        btnProblemScanner = findViewById(R.id.btnProblemScanner);

        // Set click listener for problem scanner button
        btnProblemScanner.setOnClickListener(v -> {
            goNewActivity(QRActivity.class);
        });

        // Initialize API service and vibrator
        productApiService = new ProductApiService(this);
        vibrator = (Vibrator) getSystemService(VIBRATOR_SERVICE);
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (hasCameraPermission()) {
            initCameraAndDetector();
        } else {
            requestCameraPermission();
        }
    }

    /**
     * Checks if the camera permission is granted.
     *
     * @return True if the camera permission is granted, false otherwise.
     */
    private boolean hasCameraPermission() {
        return ActivityCompat.checkSelfPermission(this, Manifest.permission.CAMERA) == PackageManager.PERMISSION_GRANTED;
    }

    /**
     * Requests the camera permission.
     */
    private void requestCameraPermission() {
        ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.CAMERA}, REQUEST_CAMERA_PERMISSION);
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        if (requestCode == REQUEST_CAMERA_PERMISSION && grantResults.length > 0 && grantResults[0] == PackageManager.PERMISSION_GRANTED) {
            recreate();
        } else {
            Toast.makeText(this, "Camera permission is required to scan QR codes", Toast.LENGTH_SHORT).show();
            startActivity(new Intent(QRActivity.this, QRManualActivity.class));
        }
    }

    /**
     * Initializes the camera and barcode detector.
     */
    private void initCameraAndDetector() {
        Log.d(TAG, "Initializing camera and detector");

        // Create barcode detector
        barcodeDetector = new BarcodeDetector.Builder(this)
                .setBarcodeFormats(Barcode.ALL_FORMATS)
                .build();

        // Create camera source
        cameraSource = new CameraSource.Builder(this, barcodeDetector)
                .setRequestedPreviewSize(1920, 1080)
                .setAutoFocusEnabled(true)
                .build();

        // Set callback for SurfaceView
        sfvQR.getHolder().addCallback(new SurfaceHolder.Callback() {
            @Override
            public void surfaceCreated(SurfaceHolder holder) {
                startCamera();
            }

            @Override
            public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
                // Do nothing
            }

            @Override
            public void surfaceDestroyed(SurfaceHolder holder) {
                stopCamera();
            }
        });

        // Set processor for barcode detection
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

                // Get the scanned QR code
                String qrCode = barcodes.valueAt(0).displayValue;
                Log.d(TAG, "QR code detected: " + qrCode);

                runOnUiThread(() -> {
                    // Check if the QR code is valid using the API service
                    productApiService.isQRValid(qrCode).observe(QRActivity.this, isValid -> {
                        if (isValid != null && isValid) {
                            // Navigate to the menu screen
                            Intent intent = new Intent(QRActivity.this, MenuActivity.class);
                            startActivity(intent);
                        } else {
                            // Vibrate and update UI with error message
                            vibrator.vibrate(100);
                            tvQR.setText("Invalid QR code\nPlease try again");
                            Log.d(TAG, "Invalid QR code, changing text and vibrating");
                        }
                    });
                });
            }
        });
    }

    /**
     * Navigates to the menu screen.
     */
    private void goNewActivity(Class<?> menuActivityClass) {
        Intent intent = new Intent(this, menuActivityClass);
        startActivity(intent);
    }

    /**
     * Starts the camera preview.
     */
    private void startCamera() {
        if (!hasCameraPermission()) {
            requestCameraPermission();
            return;
        }

        if (ActivityCompat.checkSelfPermission(this, Manifest.permission.CAMERA) != PackageManager.PERMISSION_GRANTED) {
            Log.e(TAG, "Camera permission not granted");
            return;
        }

        try {
            cameraSource.start(sfvQR.getHolder());
            Log.d(TAG, "Camera started");
        } catch (IOException e) {
            e.printStackTrace();
            Log.e(TAG, "Failed to start camera: " + e.getMessage());
        }
    }

    /**
     * Stops the camera preview.
     */
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
        if (barcodeDetector != null) {
            barcodeDetector.release();
            Log.d(TAG, "Barcode detector released");
        }
    }

    /*
     * Disable back button
     */
    @Override
    public void onBackPressed() {
        // Do nothing
    }
}