package com.github.eatup_client.ui.main;

import android.os.Bundle;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.LiveData;
import androidx.lifecycle.ViewModelProvider;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.databinding.FragmentMenuBinding;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.utils.ProductAdapter;

import java.util.List;

public class PlaceholderFragment extends Fragment {

    private static final String ARG_SECTION_NUMBER = "section_number";

    private PageViewModel pageViewModel;
    private FragmentMenuBinding binding;

    private RecyclerView recyclerView;
    private ProductAdapter adapter;
    private LiveData<List<Product>> productListLiveData;
    private ProductApiService productApiService;

    private String category;

    public static Fragment newInstance(int fragmentIndex) {
        PlaceholderFragment fragment = new PlaceholderFragment();
        Bundle bundle = new Bundle();
        bundle.putInt(ARG_SECTION_NUMBER, fragmentIndex);
        fragment.setArguments(bundle);
        return fragment;
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

        // set the category based on the index
        switch (index) {
            case 1:
                category = "Starters";
                break;
            case 2:
                category = "Appetizers";
                break;
            case 3:
                category = "Main Courses";
                break;
            case 4:
                category = "Desserts";
                break;
            case 5:
                category = "Drinks";
                break;
            default:
                throw new IllegalArgumentException("Invalid fragment index " + index);
        }
    }

    @Override
    public View onCreateView(
            @NonNull LayoutInflater inflater, ViewGroup container,
            Bundle savedInstanceState) {

        binding = FragmentMenuBinding.inflate(inflater, container, false);
        View root = binding.getRoot();

        recyclerView = binding.rvProducts;
        recyclerView.setLayoutManager(new LinearLayoutManager(getActivity()));

        adapter = new ProductAdapter(getContext());
        recyclerView.setAdapter(adapter);

        productApiService = new ProductApiService(getContext());
        productListLiveData = productApiService.getProductsByCategory(category);
        productListLiveData.observe(getViewLifecycleOwner(), productList -> {
            adapter.setProducts(productList);
            Log.i(category, "Products: " + productList.get(0).getName());
        });

        return root;
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        binding = null;
    }
}