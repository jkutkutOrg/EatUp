package com.github.eatup_client;

import android.content.Intent;
import android.os.Bundle;
import android.view.Window;
import android.view.WindowManager;
import android.widget.ImageView;

import androidx.appcompat.app.AppCompatActivity;
import androidx.fragment.app.FragmentManager;
import androidx.viewpager.widget.ViewPager;

import com.github.eatup_client.databinding.ActivityMenuBinding;
import com.github.eatup_client.ui.main.SectionsPagerAdapter;
import com.google.android.material.tabs.TabLayout;

/**
 * Activity for displaying the menu with different sections using ViewPager and TabLayout.
 * Allows navigation to the ResumeActivity and QRActivity.
 */
public class MenuActivity extends AppCompatActivity {

    private ActivityMenuBinding binding;
    private FragmentManager fragmentManager;
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
            goNewActivity(QRActivity.class);
        });

        ivResume.setOnClickListener(v -> {
            goNewActivity(ResumeActivity.class);
        });
    }

    /**
     * Navigates to the menu screen.
     */
    private void goNewActivity(Class<?> menuActivityClass) {
        Intent intent = new Intent(this, menuActivityClass);
        startActivity(intent);
    }

    /**
     * Sets up the ViewPager and Tabs using a SectionsPagerAdapter.
     */
    private void setupViewPagerAndTabs() {
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, fragmentManager);
        ViewPager viewPager = binding.viewPager;
        TabLayout tabs = binding.tabs;

        viewPager.setAdapter(sectionsPagerAdapter);

        tabs.setupWithViewPager(viewPager);
    }
}
