package com.github.eatup_client;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;
import android.widget.Button;
import android.widget.ImageView;

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
    private Context context;
    private List<OrderProduct> orderProducts;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        supportRequestWindowFeature(Window.FEATURE_NO_TITLE);

        setContentView(R.layout.activity_resume);

        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        context = this;

        btnConfirmOrder = findViewById(R.id.btnResumeOrder);
        ivBackButton = findViewById(R.id.ivBackButton);
        RecyclerView recyclerView = findViewById(R.id.rvResumeProducts);
        orderProducts = ProductRes.getInstance().getOrderProducts();

        RecyclerView.LayoutManager layoutManager = new LinearLayoutManager(this);
        recyclerView.setLayoutManager(layoutManager);

        ResumeAdapter adapter = new ResumeAdapter(orderProducts);
        recyclerView.setAdapter(adapter);

        btnConfirmOrder.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                ProductApiService productApiService = new ProductApiService(context);
                productApiService.submitOrder(orderProducts);
                Log.d(TAG, "onClick: " + orderProducts);
            }
        });

        ivBackButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // TODO: Pending to save orderProducts in shared preferences
                Intent intent = new Intent(ResumeActivity.this, MenuActivity.class);
                startActivity(intent);
            }
        });
    }
}
