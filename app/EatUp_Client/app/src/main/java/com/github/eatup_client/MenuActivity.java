package com.github.eatup_client;

import android.os.Bundle;
import android.util.Log;
import android.view.WindowManager;

import androidx.appcompat.app.AppCompatActivity;
import androidx.fragment.app.FragmentManager;
import androidx.lifecycle.LiveData;
import androidx.lifecycle.Observer;
import androidx.viewpager.widget.ViewPager;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.databinding.ActivityMenuBinding;
import com.github.eatup_client.ui.main.SectionsPagerAdapter;
import com.google.android.material.tabs.TabLayout;

public class MenuActivity extends AppCompatActivity {

    private ActivityMenuBinding binding;
    private FragmentManager fragmentManager;
    private ProductApiService mApiService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setWindowFlags();
        binding = ActivityMenuBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        fragmentManager = getSupportFragmentManager();
        setupViewPagerAndTabs();

        // Inicializar la variable mApiService
        mApiService = new ProductApiService(getApplicationContext());

        //createSession();
    }

    private void setWindowFlags() {
        getWindow().setFlags(
                WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN
        );
    }

    private void setupViewPagerAndTabs() {
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, fragmentManager);
        ViewPager viewPager = binding.viewPager;
        TabLayout tabs = binding.tabs;

        viewPager.setAdapter(sectionsPagerAdapter);
        tabs.setupWithViewPager(viewPager);
    }

    private void createSession() {
        LiveData<String> sessionLiveData = mApiService.createSession("t69");
        sessionLiveData.observe(this, new Observer<String>() {
            @Override
            public void onChanged(String s) {
                Log.d("SESSION", s);
            }
        });
    }



}