package com.github.eatup_client.api;

import android.content.Context;
import android.util.Log;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.MutableLiveData;

import com.github.eatup_client.model.Product;
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

    private static final String BASE_URL = "https://eat-up.tech/api/v1/";
    //private static final String BASE_URL = "http://159.69.216.101/api/v1/";
    private static final String ENDPOINT_PRODUCTS = "products";

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

    // Session API
    public LiveData<String> getSessionId(String simpleId) {
        MutableLiveData<String> data = new MutableLiveData<>();
        Call<String> call = mSessionService.getSessionId(simpleId);
        Log.i("ProductApiService", "getSessionId: " + call.request().url());
        call.enqueue(new Callback<String>() {
            @Override
            public void onResponse(Call<String> call, Response<String> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i("ProductApiService", "getSessionId success: " + simpleId);
                } else {
                    Log.e("ProductApiService", "getSessionId error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<String> call, Throwable t) {
                Log.e("ProductApiService", "getSessionId failure: " + t.getMessage());
            }
        });
        return data;
    }

    public LiveData<String> createSession(String tableId) {
        MutableLiveData<String> data = new MutableLiveData<>();
        Call<JsonObject> call = mSessionService.createSession(tableId);
        call.enqueue(new Callback<JsonObject>() {
            @Override
            public void onResponse(Call<JsonObject> call, Response<JsonObject> response) {
                if (response.isSuccessful()) {
                    JsonObject jsonObject = response.body();
                    String jsonString = new Gson().toJson(jsonObject);
                    data.setValue(jsonString);
                    Log.i("ProductApiService", "createSession success: " + tableId);
                } else {
                    Log.e("ProductApiService", "createSession error: " + response.code() + " " + response.message());
                }
            }

            @Override
            public void onFailure(Call<JsonObject> call, Throwable t) {
                Log.e("ProductApiService", "createSession failure: " + t.getMessage());
                t.printStackTrace();
            }
        });
        Log.i("ProductApiService", "createSession 5: " + tableId);
        return data;
    }


    public LiveData<Void> endSession(String sessionId) {
        MutableLiveData<Void> data = new MutableLiveData<>();
        Call<Void> call = mSessionService.endSession(sessionId);
        Log.i("ProductApiService", "endSession: " + call.request().url());
        call.enqueue(new Callback<Void>() {
            @Override
            public void onResponse(Call<Void> call, Response<Void> response) {
                if (response.isSuccessful()) {
                    data.setValue(null);
                    Log.i("ProductApiService", "endSession success: " + sessionId);
                } else {
                    Log.e("ProductApiService", "endSession error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<Void> call, Throwable t) {

                Log.e("ProductApiService", "endSession failure: " + t.getMessage());
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

    public interface SessionService {
        @Headers({"device: android"})
        @GET("session_id/{simple_id}")
        Call<String> getSessionId(@Path("simple_id") String simpleId);

        @Headers({"device: android"})
        @POST("session/{table_id}")
        Call<JsonObject> createSession(@Path("table_id") String tableId);

        @Headers({"device: android"})
        @PATCH("session/{session_id}/end")
        Call<Void> endSession(@Path("session_id") String sessionId);
    }

}