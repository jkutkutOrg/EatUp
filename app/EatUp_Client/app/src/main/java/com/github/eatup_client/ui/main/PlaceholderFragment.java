package com.github.eatup_client.ui.main;

import android.os.Bundle;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.LiveData;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.databinding.FragmentMenuBinding;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.utils.ProductAdapter;

import java.util.List;

/**
 * A placeholder fragment containing a category-specific menu.
 */
public class PlaceholderFragment extends Fragment {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private static final String TAG = PlaceholderFragment.class.getSimpleName();

    private FragmentMenuBinding binding;
    private RecyclerView recyclerView;
    private ProductAdapter adapter;
    private ProductApiService productApiService;
    private String category;

    public static PlaceholderFragment newInstance(int fragmentIndex) {
        PlaceholderFragment fragment = new PlaceholderFragment();
        Bundle bundle = new Bundle();
        bundle.putInt(ARG_SECTION_NUMBER, fragmentIndex);
        fragment.setArguments(bundle);
        return fragment;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        int index = 1;
        if (getArguments() != null) {
            index = getArguments().getInt(ARG_SECTION_NUMBER);
        }

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

        adapter = new ProductAdapter(requireContext());

        productApiService = new ProductApiService(requireContext());

        loadProductsFromAPI();
    }

    /**
     * Loads the products for the current category from the API and updates the adapter.
     */
    private void loadProductsFromAPI() {
        LiveData<List<Product>> productListLiveData = productApiService.getProductsByCategory(category);
        productListLiveData.observe(this, productList -> {
            adapter.setProducts(productList);
            Log.d(TAG, "Loaded products from API");
        });
    }

    @Override
    public View onCreateView(@NonNull LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        binding = FragmentMenuBinding.inflate(inflater, container, false);
        initializeRecyclerView();
        return binding.getRoot();
    }

    /**
     * Initializes the RecyclerView and sets up the adapter.
     */
    private void initializeRecyclerView() {
        recyclerView = binding.rvProducts;
        recyclerView.setLayoutManager(new LinearLayoutManager(requireContext()));
        recyclerView.setAdapter(adapter);
    }
}
