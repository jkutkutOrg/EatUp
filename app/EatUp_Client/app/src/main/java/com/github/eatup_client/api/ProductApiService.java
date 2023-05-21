package com.github.eatup_client.api;

import android.content.Context;
import android.util.Log;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.MutableLiveData;

import com.github.eatup_client.model.Product;
import com.github.eatup_client.model.Session;
import com.google.gson.Gson;
import com.google.gson.JsonObject;

import java.util.List;

import okhttp3.Cache;
import okhttp3.OkHttpClient;
import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Response;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;
import retrofit2.http.GET;
import retrofit2.http.Headers;
import retrofit2.http.PATCH;
import retrofit2.http.POST;
import retrofit2.http.Path;
import retrofit2.http.Query;

public class ProductApiService {

    //private static final String BASE_URL = "https://eat-up.tech/api/v1/";
    private static final String BASE_URL = "http://159.69.216.101/api/v1/";
    private static final String ENDPOINT_PRODUCTS = "products";
    private static final String ENDPOINT_SESSIONS = "sessions";

    private static ProductApiService sInstance;
    private final ProductService mProductService;
    private final SessionService mSessionService;
    private Context mContext;

    public ProductApiService(Context context) {

        mContext = context;

        OkHttpClient client = new OkHttpClient.Builder()
                .cache(new Cache(mContext.getCacheDir(), 10 * 1024 * 1024)) // 10 MB cache
                .build();
        Retrofit retrofit = new Retrofit.Builder()
                .baseUrl(BASE_URL)
                .client(client)
                .addConverterFactory(GsonConverterFactory.create())
                .build();

        mProductService = retrofit.create(ProductService.class);
        mSessionService = retrofit.create(SessionService.class);
    }

    public static synchronized ProductApiService getInstance() {
        if (sInstance == null) {
            sInstance = new ProductApiService(null);
        }
        return sInstance;
    }

    public LiveData<List<Product>> getProductsByCategory(String category) {
        MutableLiveData<List<Product>> data = new MutableLiveData<>();
        Call<List<Product>> call = mProductService.getProductsByCategory(category);
        Log.i("ProductApiService", "getProductsByCategory: " + call.request().url());
        call.enqueue(new Callback<List<Product>>() {
            @Override
            public void onResponse(Call<List<Product>> call, Response<List<Product>> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i("ProductApiService", "getProductsByCategory success: " + category);
                } else {
                    Log.e("ProductApiService", "getProductsByCategory error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Product>> call, Throwable t) {
                Log.e("ProductApiService", "getProductsByCategory failure: " + t.getMessage());
            }
        });
        return data;
    }

    // Load Session (all)
    public LiveData<List<Session>> loadSessions() {
        MutableLiveData<List<Session>> data = new MutableLiveData<>();
        Call<List<Session>> call = mSessionService.getSessions();
        //Log.i("SessionController", "loadSessions: " + call.request().url());
        call.enqueue(new Callback<List<Session>>() {
            @Override
            public void onResponse(Call<List<Session>> call, Response<List<Session>> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i("SessionController", "Success: " + new Gson().toJson(response.body()));
                } else {
                    Log.e("SessionController", "Error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Session>> call, Throwable t) {
                Log.e("SessionController", "Failure: " + t.getMessage());
            }
        });

        return data;
    }


    // Retrofit interface
    public interface ProductService {
        @Headers({"device: android"})
        @GET(ENDPOINT_PRODUCTS)
        Call<List<Product>> getProductsByCategory(@Query("category") String category);
    }

    // Retrofit interface
    public interface SessionService {
        @Headers({"device: android"})
        @GET(ENDPOINT_SESSIONS)
        Call<List<Session>> getSessions();
    }


}