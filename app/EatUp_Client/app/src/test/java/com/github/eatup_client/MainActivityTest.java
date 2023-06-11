/**
 * This class contains unit tests for the MainActivity class.
 */
package com.github.eatup_client;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

import android.content.Context;
import android.net.ConnectivityManager;
import android.net.NetworkInfo;
import android.os.Build;

import com.github.eatup_client.api.ProductApiService;

import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.robolectric.RobolectricTestRunner;
import org.robolectric.annotation.Config;

/**
 * Unit tests for the MainActivity class.
 */
@RunWith(RobolectricTestRunner.class)
@Config(sdk = {Build.VERSION_CODES.O_MR1})
public class MainActivityTest {

    private MainActivity mainActivity;
    private Context mockContext;
    private ConnectivityManager mockConnectivityManager;
    private NetworkInfo mockNetworkInfo;

    /**
     * Sets up the test environment before each test method.
     */
    @Before
    public void setup() {
        mainActivity = new MainActivity();
        mainActivity.productApiService = mock(ProductApiService.class);

        mockContext = mock(Context.class);
        mockConnectivityManager = mock(ConnectivityManager.class);
        mockNetworkInfo = mock(NetworkInfo.class);

        when(mockContext.getSystemService(Context.CONNECTIVITY_SERVICE)).thenReturn(mockConnectivityManager);
        when(mockConnectivityManager.getActiveNetworkInfo()).thenReturn(mockNetworkInfo);

        mainActivity.setConnectivityManager(mockConnectivityManager);
    }

    /**
     * Tests the isNetworkAvailable() method when network is available.
     */
    @Test
    public void testIsNetworkAvailable_NetworkAvailable() {
        // Set up mock behavior
        when(mockNetworkInfo.isConnected()).thenReturn(true);

        // Execute the method under test
        boolean result = mainActivity.isNetworkAvailable();

        // Verify the result
        assertTrue(result);
    }

    /**
     * Tests the isNetworkAvailable() method when network is not available.
     */
    @Test
    public void testIsNetworkAvailable_NetworkNotAvailable() {
        // Set up mock behavior
        when(mockNetworkInfo.isConnected()).thenReturn(false);

        // Execute the method under test
        boolean result = mainActivity.isNetworkAvailable();

        // Verify the result
        assertFalse(result);
    }
}
