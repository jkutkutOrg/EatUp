package com.github.eatup_client;

import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.WindowManager;
import android.widget.Button;

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
}
