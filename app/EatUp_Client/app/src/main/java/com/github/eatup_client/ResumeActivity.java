package com.github.eatup_client;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.os.Vibrator;
import android.util.Log;
import android.view.View;
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
import com.github.eatup_client.model.ProductRes;
import com.github.eatup_client.utils.ResumeAdapter;

import java.util.List;

public class ResumeActivity extends AppCompatActivity {

    private static final String TAG = "ResumeActivity";
    private Button btnConfirmOrder;
    private ImageView ivBackButton;
    private TextView tvResume;
    private List<OrderProduct> orderProducts;
    private ProductApiService productApiService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        requestWindowFeature(Window.FEATURE_NO_TITLE);
        setContentView(R.layout.activity_resume);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        Context context = this;
        ProductRes productRes = ProductRes.getInstance();
        productApiService = new ProductApiService(context);

        btnConfirmOrder = findViewById(R.id.btnResumeOrder);
        ivBackButton = findViewById(R.id.ivBackButton);
        tvResume = findViewById(R.id.tvResume);

        RecyclerView recyclerView = findViewById(R.id.rvResumeProducts);
        orderProducts = ProductRes.getInstance().getOrderProducts();

        // TODO: Pending to change method
        tvResume.setText(productRes.getTotalPrice() + "$");

        RecyclerView.LayoutManager layoutManager = new LinearLayoutManager(this);
        recyclerView.setLayoutManager(layoutManager);

        ResumeAdapter adapter = new ResumeAdapter(orderProducts);
        recyclerView.setAdapter(adapter);

        btnConfirmOrder.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Check if orderProducts is empty
                if (orderProducts.isEmpty()) {
                    Toast.makeText(context, "You have to add products to your order", Toast.LENGTH_SHORT).show();
                    goNewActivity(MenuActivity.class);
                } else {
                    submitOrder();
                }

                // Log orderProducts
                Log.d(TAG, "onClick: " + orderProducts);
            }
        });

        ivBackButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // TODO: Pending to save orderProducts in shared preferences
                goNewActivity(MenuActivity.class);
            }
        });
    }

    private void submitOrder() {
        productApiService.submitOrder(orderProducts);

        orderProducts.clear();
        goNewActivity(MenuActivity.class);

        Vibrator vibrator = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        vibrator.vibrate(500);

        Toast.makeText(this, "Order confirmed", Toast.LENGTH_SHORT).show();
    }

    private void goNewActivity(Class<?> menuActivityClass) {
        Intent intent = new Intent(this, menuActivityClass);
        startActivity(intent);
    }
}
