package com.github.eatup_client;

import android.content.Intent;
import android.os.Bundle;
import android.view.Window;
import android.view.WindowManager;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;
import androidx.fragment.app.FragmentManager;
import androidx.lifecycle.Observer;
import androidx.viewpager.widget.ViewPager;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.databinding.ActivityMenuBinding;
import com.github.eatup_client.model.OrderItem;
import com.github.eatup_client.ui.main.SectionsPagerAdapter;
import com.google.android.material.tabs.TabLayout;

import java.util.List;

public class MenuActivity extends AppCompatActivity {

    private ActivityMenuBinding binding;
    private FragmentManager fragmentManager;
    private ProductApiService productApiService;
    private TextView tvTotalBill;
    private ImageView ivBackButton;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        supportRequestWindowFeature(Window.FEATURE_NO_TITLE);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        binding = ActivityMenuBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        fragmentManager = getSupportFragmentManager();
        setupViewPagerAndTabs();

        tvTotalBill = findViewById(R.id.tvTotalBill);
        ivBackButton = findViewById(R.id.ivBackButton);

        ivBackButton.setOnClickListener(v -> {
            Intent intent = new Intent(MenuActivity.this, QRActivity.class);
            startActivity(intent);
        });

        productApiService = ProductApiService.getInstance(getApplicationContext());
        productApiService.getOrdersBySessionUUID().observe(this, new Observer<List<OrderItem>>() {
            @Override
            public void onChanged(List<OrderItem> orderItems) {
                double totalBill = productApiService.getCartTotalPrice();

                String totalBillString = String.format("%.2f", totalBill);
                tvTotalBill.setText(totalBillString);
            }
        });
    }

    private void setupViewPagerAndTabs() {
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, fragmentManager);
        ViewPager viewPager = binding.viewPager;
        TabLayout tabs = binding.tabs;

        viewPager.setAdapter(sectionsPagerAdapter);
        tabs.setupWithViewPager(viewPager);
    }

}
