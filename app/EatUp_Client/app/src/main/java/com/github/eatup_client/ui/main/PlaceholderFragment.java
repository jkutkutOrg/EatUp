package com.github.eatup_client.ui.main;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.Observer;
import androidx.lifecycle.ViewModelProvider;

import com.github.eatup_client.databinding.FragmentMenuBinding;
import com.github.eatup_client.fragments.AppetizersFragment;
import com.github.eatup_client.fragments.DessertsFragment;
import com.github.eatup_client.fragments.DrinksFragment;
import com.github.eatup_client.fragments.MainCoursesFragment;
import com.github.eatup_client.fragments.StartersFragment;

public class PlaceholderFragment extends Fragment {

    private static final String ARG_SECTION_NUMBER = "section_number";

    private PageViewModel pageViewModel;
    private FragmentMenuBinding binding;

    public static Fragment newInstance(int fragmentIndex) {

        switch (fragmentIndex) {
            case 1:
                return new StartersFragment();
            case 2:
                return new AppetizersFragment();
            case 3:
                return new MainCoursesFragment();
            case 4:
                return new DessertsFragment();
            case 5:
                return new DrinksFragment();
            default:
                throw new IllegalArgumentException("Invalid fragment index");
        }
    }


    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        pageViewModel = new ViewModelProvider(this).get(PageViewModel.class);
        int index = 1;
        if (getArguments() != null) {
            index = getArguments().getInt(ARG_SECTION_NUMBER);
        }
        pageViewModel.setIndex(index);
    }

    @Override
    public View onCreateView(
            @NonNull LayoutInflater inflater, ViewGroup container,
            Bundle savedInstanceState) {

        binding = FragmentMenuBinding.inflate(inflater, container, false);
        View root = binding.getRoot();

        final TextView textView = binding.sectionLabel;
        pageViewModel.getText().observe(getViewLifecycleOwner(), new Observer<String>() {
            @Override
            public void onChanged(@Nullable String s) {
                textView.setText(s);
            }
        });
        return root;
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        binding = null;
    }
}