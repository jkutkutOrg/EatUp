package com.github.eatup_client;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.os.Vibrator;
import android.util.Log;
import android.view.Window;
import android.view.WindowManager;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.model.OrderProduct;
import com.github.eatup_client.model.MVCManager;
import com.github.eatup_client.utils.ResumeAdapter;

import java.util.List;

/**
 * Activity to display the resume of the order before the confirmation
 */
public class ResumeActivity extends AppCompatActivity {

    private static final String TAG = ResumeActivity.class.getSimpleName();
    private Button btnConfirmOrder;
    private ImageView ivBackButton;
    private TextView tvResume;
    private List<OrderProduct> orderProducts;
    private ProductApiService productApiService;
    private MVCManager mvcManager;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        requestWindowFeature(Window.FEATURE_NO_TITLE);
        setContentView(R.layout.activity_resume);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        Context context = this;
        mvcManager = MVCManager.getInstance();
        productApiService = new ProductApiService(context);

        // Initialize views
        btnConfirmOrder = findViewById(R.id.btnResumeOrder);
        ivBackButton = findViewById(R.id.ivBackButton);
        tvResume = findViewById(R.id.tvResume);

        RecyclerView recyclerView = findViewById(R.id.rvResumeProducts);
        orderProducts = mvcManager.getOrderProducts();

        // Display the total price of the order
        tvResume.setText(String.format("Total: %s €", mvcManager.getTotalPrice()));
        RecyclerView.LayoutManager layoutManager = new LinearLayoutManager(this);
        recyclerView.setLayoutManager(layoutManager);

        ResumeAdapter adapter = new ResumeAdapter(orderProducts);
        recyclerView.setAdapter(adapter);

        // Action when the confirm order button is clicked
        btnConfirmOrder.setOnClickListener(v -> {
            if (orderProducts.isEmpty()) {
                Toast.makeText(context, R.string.resume_activity_empty_order, Toast.LENGTH_SHORT).show();
                goNewActivity(MenuActivity.class);
            } else {
                mvcManager.clear();
                submitOrder();
            }

            Log.d(TAG, "onClick: " + orderProducts);
        });

        // Action when the back button is clicked
        ivBackButton.setOnClickListener(v -> goNewActivity(MenuActivity.class));
    }

    /**
     * Submits the order to the product API service and displays a confirmation message.
     */
    private void submitOrder() {
        productApiService.submitOrder(orderProducts);
        goNewActivity(MenuActivity.class);

        Vibrator vibrator = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        if (vibrator != null) {
            vibrator.vibrate(500);
        }

        Toast.makeText(this, "Order confirmed", Toast.LENGTH_SHORT).show();
    }

    /**
     * Starts a new activity.
     *
     * @param menuActivityClass The class of the activity to start.
     */
    private void goNewActivity(Class<?> menuActivityClass) {
        Intent intent = new Intent(this, menuActivityClass);
        startActivity(intent);
    }
}