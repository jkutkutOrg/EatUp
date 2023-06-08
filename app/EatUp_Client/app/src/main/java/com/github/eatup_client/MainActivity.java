package com.github.eatup_client;

import android.content.Context;
import android.content.Intent;
import android.net.ConnectivityManager;
import android.net.NetworkInfo;
import android.os.Bundle;
import android.util.Log;
import android.view.WindowManager;
import android.widget.Button;

import androidx.appcompat.app.AlertDialog;
import androidx.appcompat.app.AppCompatActivity;

import com.github.eatup_client.api.ProductApiService;

/**
 * The main activity of the app.
 * Displays a "Get Started" button and navigates to the QRActivity when clicked.
 */
public class MainActivity extends AppCompatActivity {

    private static final String TAG = "MainActivity";
    private Button btnGetStarted;
    private ProductApiService productApiService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setFullScreenWindowFlags();
        setContentView(R.layout.activity_main);

        productApiService = new ProductApiService(this);

        if (!isNetworkAvailable()) {
            showNoInternetDialog();
        }

        productApiService.loadSessions();

        btnGetStarted = findViewById(R.id.btnGetStarted);

        btnGetStarted.setOnClickListener(v -> {
            goNewActivity(QRActivity.class);
        });
    }

    /**
     * Navigates to the menu screen.
     */
    private void goNewActivity(Class<?> menuActivityClass) {
        Log.d(TAG, "goNewActivity: Navigating to " + menuActivityClass.getSimpleName());
        Intent intent = new Intent(this, menuActivityClass);
        startActivity(intent);
    }

    /**
     * Sets the window flags to enable fullscreen.
     */
    private void setFullScreenWindowFlags() {
        getWindow().setFlags(
                WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN
        );
    }

    /**
     * Checks if the device is connected to the internet.
     */
    private boolean isNetworkAvailable() {
        ConnectivityManager connectivityManager = (ConnectivityManager) getSystemService(Context.CONNECTIVITY_SERVICE);
        if (connectivityManager != null) {
            NetworkInfo networkInfo = connectivityManager.getActiveNetworkInfo();
            return networkInfo != null && networkInfo.isConnected();
        }
        return false;
    }

    /**
     * Shows a dialog if the device is not connected to the internet.
     */
    private void showNoInternetDialog() {
        AlertDialog.Builder builder = new AlertDialog.Builder(this);

        builder.setTitle("No Internet Connection");
        builder.setMessage("Please check your internet connection and try again");
        builder.setPositiveButton("Retry", (dialog, which) -> {
            if (isNetworkAvailable()) {
                setContentView(R.layout.activity_main);
            } else {
                showNoInternetDialog();
            }
        });
        builder.setNegativeButton("Exit", (dialog, which) -> finish());
        builder.setCancelable(false);
        builder.show();
    }
}
