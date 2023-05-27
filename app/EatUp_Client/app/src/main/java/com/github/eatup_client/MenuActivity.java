package com.github.eatup_client;

import android.os.Bundle;
import android.view.Window;
import android.view.WindowManager;

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

        productApiService = new ProductApiService(getApplicationContext());
    }

    private void setupViewPagerAndTabs() {
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, fragmentManager);
        ViewPager viewPager = binding.viewPager;
        TabLayout tabs = binding.tabs;

        viewPager.setAdapter(sectionsPagerAdapter);
        tabs.setupWithViewPager(viewPager);
    }

}