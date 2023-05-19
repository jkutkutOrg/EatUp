package com.github.eatup_client.ui.main;

import android.content.SharedPreferences;
import android.os.Bundle;
import android.preference.PreferenceManager;
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
import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;

import java.util.List;
import java.util.concurrent.TimeUnit;

public class PlaceholderFragment extends Fragment {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private static final String TAG = "PlaceholderFragment";
    private static final TypeToken<List<Product>> LIST_TYPE_TOKEN = new TypeToken<List<Product>>(){};

    private FragmentMenuBinding binding;
    private RecyclerView recyclerView;
    private ProductAdapter adapter;
    private LiveData<List<Product>> productListLiveData;
    private ProductApiService productApiService;
    private String category;
    private SharedPreferences prefs;
    private Gson gson;

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
        int index = 1;
        if (getArguments() != null) {
            index = getArguments().getInt(ARG_SECTION_NUMBER);
        }

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

        prefs = PreferenceManager.getDefaultSharedPreferences(requireContext());
        gson = new Gson();
    }

    @Override
    public View onCreateView(@NonNull LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {

        binding = FragmentMenuBinding.inflate(inflater, container, false);
        initializeRecyclerView();

        productApiService = new ProductApiService(requireContext());
        String cachedProductsJson = prefs.getString(category, null);
        List<Product> cachedProducts = null;

        if (cachedProductsJson != null) {
            long lastUpdateTime = prefs.getLong(category + "_last_update_time", 0);
            if (System.currentTimeMillis() - lastUpdateTime < TimeUnit.MINUTES.toMillis(15)) {
                // Use cached data if it is less than 30 minutes old
                cachedProducts = gson.fromJson(cachedProductsJson, LIST_TYPE_TOKEN.getType());
                adapter.setProducts(cachedProducts);
                Log.d(TAG, "Loaded products from cache");
            }
        }

        if (cachedProducts == null) {
            Log.d(TAG, "Loaded products from API");
            Log.d(TAG, "category: " + category + ", cachedProductsJson: " + cachedProductsJson);
            productListLiveData = productApiService.getProductsByCategory(category);
            productListLiveData.observe(getViewLifecycleOwner(), productList -> {
                adapter.setProducts(productList);
                String productsJson = gson.toJson(productList);
                prefs.edit().putString(category, productsJson).apply();
                prefs.edit().putLong(category + "_last_update_time", System.currentTimeMillis()).apply();
            });
        }

        return binding.getRoot();
    }


    private void initializeRecyclerView() {
        recyclerView = binding.rvProducts;
        recyclerView.setLayoutManager(new LinearLayoutManager(requireContext()));
        adapter = new ProductAdapter(requireContext());
        recyclerView.setAdapter(adapter);
    }
}
