package com.github.eatup_client;

import android.content.Intent;
import android.os.Bundle;
import android.view.Window;
import android.view.WindowManager;
import android.widget.ImageView;

import androidx.appcompat.app.AppCompatActivity;
import androidx.fragment.app.FragmentManager;
import androidx.viewpager.widget.ViewPager;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.databinding.ActivityMenuBinding;
import com.github.eatup_client.ui.main.SectionsPagerAdapter;
import com.google.android.material.tabs.TabLayout;

public class MenuActivity extends AppCompatActivity {

    private ActivityMenuBinding binding;
    private FragmentManager fragmentManager;
    private ProductApiService productApiService;
    private ImageView ivResume;
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

        ivResume = findViewById(R.id.ivResume);
        ivBackButton = findViewById(R.id.ivBackButton);

        ivBackButton.setOnClickListener(v -> {
            Intent intent = new Intent(MenuActivity.this, QRActivity.class);
            startActivity(intent);
        });

        ivResume.setOnClickListener(v -> {
            Intent intent = new Intent(MenuActivity.this, ResumeActivity.class);
            startActivity(intent);
        });

        productApiService = ProductApiService.getInstance(getApplicationContext());
    }

    private void setupViewPagerAndTabs() {
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, fragmentManager);
        ViewPager viewPager = binding.viewPager;
        TabLayout tabs = binding.tabs;

        viewPager.setAdapter(sectionsPagerAdapter);
        tabs.setupWithViewPager(viewPager);
    }


}
