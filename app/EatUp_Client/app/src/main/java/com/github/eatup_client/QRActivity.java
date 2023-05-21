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

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;
import androidx.lifecycle.LiveData;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.model.Session;
import com.google.android.gms.vision.CameraSource;
import com.google.android.gms.vision.Detector;
import com.google.android.gms.vision.barcode.Barcode;
import com.google.android.gms.vision.barcode.BarcodeDetector;

import java.io.IOException;
import java.util.List;
import java.util.regex.Pattern;

public class QRActivity extends AppCompatActivity {

    private SurfaceView sfvQR;
    private TextView tvQR;
    private Button btnProblemScanner;
    private BarcodeDetector barcodeDetector;
    private CameraSource cameraSource;
    private LiveData<List<Session>> sessionListLiveData;
    private ProductApiService productApiService;
    private static final int REQUEST_CAMERA_PERMISSION = 201;
    private static final Pattern QR_PATTERN = Pattern.compile("[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}");

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_qr);
        this.getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

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
        sessionListLiveData = productApiService.loadSessions();

        initCameraAndDetector();
    }

    private void initCameraAndDetector() {
        Log.d("QRActivity", "Scanner started");

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
                try {
                    if (ActivityCompat.checkSelfPermission(QRActivity.this, Manifest.permission.CAMERA) == PackageManager.PERMISSION_GRANTED) {
                        cameraSource.start(sfvQR.getHolder());
                    } else {
                        ActivityCompat.requestPermissions(QRActivity.this, new String[]{Manifest.permission.CAMERA}, REQUEST_CAMERA_PERMISSION);
                    }

                } catch (IOException e) {
                    e.printStackTrace();
                }
            }

            @Override
            public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
            }

            @Override
            public void surfaceDestroyed(SurfaceHolder holder) {
                cameraSource.stop();
            }
        });

        barcodeDetector.setProcessor(new Detector.Processor<Barcode>() {
            @Override
            public void release() {
                Log.d("QRActivity", "Barcode scanner stopped to prevent memory leaks");
            }

            @Override
            public void receiveDetections(@NonNull Detector.Detections<Barcode> detections) {
                final SparseArray<Barcode> barcodes = detections.getDetectedItems();
                if (barcodes.size() == 0) {
                    return;
                }

                tvQR.post(() -> {
                    String qrCode = barcodes.valueAt(0).displayValue;
                    boolean isValid = isQRValid(qrCode);
                    if (isValid) {
                        Intent intent = new Intent(QRActivity.this, MainActivity.class);
                        startActivity(intent);
                    } else {
                        ((Vibrator) getSystemService(VIBRATOR_SERVICE)).vibrate(100);
                        tvQR.setText("Invalid QR code\nPlease try again");
                        Log.d("QRActivity", "Invalid QR code, change text and vibrate");
                    }
                });
            }
        });
    }

    private boolean isQRValid(String qrCode) {
        List<Session> sessionList = sessionListLiveData.getValue();
        if (sessionList != null) {
            for (Session session : sessionList) {
                Log.d("QRActivity", "Session ID: " + session.getId());
                if (session.getId().equals(qrCode)) {
                    Log.d("QRActivity", "Session ID matches QR code");
                    return true;
                }
            }
        }
        Log.d("QRActivity", "Session ID does not match QR code");
        return false;
    }


    @Override
    protected void onPause() {
        super.onPause();
        cameraSource.release();
    }

    @Override
    protected void onResume() {
        super.onResume();
        initCameraAndDetector();
    }
}